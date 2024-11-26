use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    default_widgets::Overflow,
    geometry::{Pos, Size},
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::{Hex, List, State, Value},
    widgets::Elements,
};
use serde::{Deserialize, Serialize};
use syntect::highlighting::Theme;

use super::syntax_highlighter::{highlight, Instruction, Parser};

const TEMPLATE: &str = "./src/components/templates/response_renderer.aml";

enum ScrollDirection {
    Up,
    Down,
}

pub struct ResponseRenderer {
    #[allow(unused)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
    cursor: Pos,
    foreground: Hex,
    background: Hex,
    instructions: Vec<Instruction>,
    text_filter: TextFilter,
    theme: Option<Theme>,
}

fn scroll(
    state: &mut ResponseRendererState,
    mut elements: Elements<'_, '_>,
    context: Context<'_, ResponseRendererState>,
    direction: ScrollDirection,
) {
    elements
        .by_attribute("id", "container")
        .each(|el, _attributes| {
            let overflow = el.to::<Overflow>();

            let scroll_amount = context.viewport.size().height / 2;
            let scroll_position = *state.scroll_position.to_ref();
            let new_scroll_position = match direction {
                ScrollDirection::Up => scroll_position.saturating_sub(scroll_amount),
                ScrollDirection::Down => scroll_position + scroll_amount,
            };

            state.scroll_position.set(new_scroll_position);

            match direction {
                ScrollDirection::Up => overflow.scroll_up_by(scroll_amount as i32),
                ScrollDirection::Down => overflow.scroll_down_by(scroll_amount as i32),
            }
        });
}

impl ResponseRenderer {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
        ident: String,
    ) -> anyhow::Result<()> {
        let id = builder.register_component(
            ident.clone(),
            TEMPLATE,
            ResponseRenderer::new(ids.clone()),
            ResponseRendererState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(ident, id);

            new_map
        });

        Ok(())
    }

    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        ResponseRenderer {
            component_ids,
            cursor: Pos::ZERO,
            foreground: Hex::from((255, 255, 255)),
            background: Hex::BLACK,
            instructions: vec![],
            text_filter: TextFilter {
                ..Default::default()
            },
            theme: None,
        }
    }

    fn update_cursor(
        &mut self,
        state: &mut ResponseRendererState,
        overflow: &mut Overflow,
        size: Size,
    ) {
        // Make sure there are enough lines and spans
        while self.cursor.y as usize >= state.lines.len() {
            state.lines.push_back(Line::empty());
        }

        {
            let mut lines = state.lines.to_mut();
            let line = lines.get_mut(self.cursor.y as usize).unwrap();

            let spans = &mut line.to_mut().spans;
            while self.cursor.x as usize > spans.len() {
                spans.push_back(Span::empty());
            }
        }

        let mut screen_cursor = self.cursor - overflow.offset();

        if screen_cursor.y < 0 {
            overflow.scroll_up_by(-screen_cursor.y);
            screen_cursor.y = 0;
        }

        if screen_cursor.y >= size.height as i32 {
            let offset = screen_cursor.y + 1 - size.height as i32;
            overflow.scroll_down_by(offset);
            screen_cursor.y = size.height as i32 - 1;
        }

        state.screen_cursor_x.set(screen_cursor.x);
        state.screen_cursor_y.set(screen_cursor.y);
        state.buf_cursor_x.set(self.cursor.x);
        state.buf_cursor_y.set(self.cursor.y);
    }

    pub fn apply_inst(
        &mut self,
        inst: &Instruction,
        state: &mut ResponseRendererState,
        elements: &mut Elements<'_, '_>,
    ) {
        state.current_instruction.set(Some(format!("{inst:?}")));
        elements.by_tag("overflow").first(|el, _| {
            let size = el.size();
            let vp = el.to::<Overflow>();

            match inst {
                Instruction::MoveCursor(x, y) => {
                    self.cursor.x = *x as i32;
                    self.cursor.y = *y as i32;
                    self.update_cursor(state, vp, size);
                }
                Instruction::Type(c, bold) => {
                    {
                        let mut lines = state.lines.to_mut();
                        let line = lines.get_mut(self.cursor.y as usize);

                        if line.is_none() {
                            return;
                        }

                        let line = line.unwrap();
                        let mut line = line.to_mut();
                        // let spans = line.spans.len();
                        line.spans.insert(
                            self.cursor.x as usize,
                            Span::new(*c, self.foreground, self.background, *bold),
                        );
                        self.cursor.x += 1;
                    }

                    self.update_cursor(state, vp, size);
                }
                Instruction::SetForeground(hex) => self.foreground = *hex,
                Instruction::SetBackground(hex) => self.background = *hex,
                Instruction::Newline { x } => {
                    self.cursor.x = *x;
                    self.cursor.y += 1;
                    self.update_cursor(state, vp, size);
                }
                Instruction::SetX(x) => {
                    self.cursor.x = *x;
                    self.update_cursor(state, vp, size);
                }
                Instruction::Pause(_) => unreachable!(),
                Instruction::Wait => state.waiting.set(true.to_string()),
                Instruction::HideCursor => {
                    state.show_cursor.set(false);
                }
            }
        });
    }
}

#[derive(State)]
pub struct Line {
    spans: Value<List<Span>>,
}

impl Line {
    pub fn empty() -> Self {
        Self {
            spans: List::empty(),
        }
    }
}

#[derive(State)]
struct Span {
    text: Value<char>,
    bold: Value<bool>,
    foreground: Value<Hex>,
    background: Value<Hex>,
    original_background: Value<Option<Hex>>,
}

impl Span {
    pub fn new(c: char, foreground: Hex, background: Hex, bold: bool) -> Self {
        Self {
            text: c.into(),
            foreground: foreground.into(),
            background: background.into(),
            original_background: None.into(),
            bold: bold.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            text: ' '.into(),
            foreground: Hex::from((255, 255, 255)).into(),
            background: Hex::from((0, 0, 0)).into(),
            original_background: None.into(),
            bold: false.into(),
        }
    }
}

#[derive(Default, State)]
pub struct ResponseRendererState {
    scroll_position: Value<usize>,
    pub doc_height: Value<usize>,
    pub screen_cursor_x: Value<i32>,
    pub screen_cursor_y: Value<i32>,
    pub buf_cursor_x: Value<i32>,
    pub buf_cursor_y: Value<i32>,
    pub lines: Value<List<Line>>,
    pub current_instruction: Value<Option<String>>,
    pub title: Value<String>,
    pub waiting: Value<String>,
    pub show_cursor: Value<bool>,
    pub response: Value<String>,
    pub response_background: Value<String>,
}

impl ResponseRendererState {
    pub fn new() -> Self {
        ResponseRendererState {
            response: "".to_string().into(),
            scroll_position: 0.into(),
            doc_height: 1.into(),
            screen_cursor_x: 0.into(),
            screen_cursor_y: 0.into(),
            buf_cursor_x: 0.into(),
            buf_cursor_y: 0.into(),
            lines: List::from_iter(vec![Line::empty()]),
            current_instruction: None.into(),
            title: "".to_string().into(),
            waiting: false.to_string().into(),
            show_cursor: true.into(),
            response_background: "#000000".to_string().into(),
        }
    }
}

impl Component for ResponseRenderer {
    type State = ResponseRendererState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        #[allow(clippy::single_match)]
        match event.code {
            anathema::component::KeyCode::Esc => {
                context.set_focus("id", "app");
            }

            anathema::component::KeyCode::Char(char) => {
                match event.ctrl {
                    true => {
                        match char {
                            'd' => scroll(state, elements, context, ScrollDirection::Down),
                            'u' => scroll(state, elements, context, ScrollDirection::Up),

                            'p' => {
                                // move to previous find
                                let current_index = self.text_filter.nav_index;
                                let line = if current_index == 0 {
                                    self.text_filter.indexes.len().saturating_sub(1)
                                } else {
                                    current_index.saturating_sub(1)
                                };

                                self.text_filter.nav_index = line;
                                let line = self.text_filter.indexes.get(line).unwrap_or(&0);

                                scroll_to_line(state, &mut elements, context, *line);
                            }

                            'n' => {
                                // move to previous find
                                let current_index = self.text_filter.nav_index;
                                let last_index = self.text_filter.indexes.len().saturating_sub(1);
                                let line = if current_index == last_index {
                                    self.text_filter.indexes.first()
                                } else {
                                    self.text_filter.indexes.get(current_index + 1)
                                };

                                let line = line.unwrap_or(&0);

                                self.text_filter.nav_index = *line;

                                scroll_to_line(state, &mut elements, context, *line);
                            }
                            _ => {}
                        }
                    }

                    false => {}
                }
            }

            _ => {}
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let response_renderer_message = serde_json::from_str::<ResponseRendererMessages>(&message);

        #[allow(clippy::single_match)]
        match response_renderer_message {
            Ok(message) => match message {
                ResponseRendererMessages::ResponseUpdate((response, extension, theme)) => {
                    loop {
                        if state.lines.len() == 0 {
                            break;
                        }

                        state.lines.remove(0);
                    }

                    let (highlighted_lines, parsed_theme) = highlight(&response, &extension, theme);

                    // NOTE: Maybe remove this if its not useful
                    self.theme = Some(parsed_theme.clone());

                    if let Some(color) = parsed_theme.settings.background {
                        let hex_color = format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b);
                        state.response_background.set(hex_color);
                    }

                    self.instructions = Parser::new(highlighted_lines).instructions();

                    for instruction in self.instructions.clone() {
                        self.apply_inst(&instruction, state, &mut elements);
                    }

                    state.response.set(response);
                }

                ResponseRendererMessages::FilterUpdate(filter) => {
                    self.text_filter = filter;

                    // Go to the first search match
                    let default_index = 0;
                    let first_index = self.text_filter.indexes.first().unwrap_or(&default_index);
                    scroll_to_line(state, &mut elements, context, *first_index);
                    highlight_matches(
                        state,
                        &mut self.text_filter.indexes,
                        &self.text_filter.filter,
                    );
                }
            },
            Err(_) => {}
        }
    }
}

fn highlight_matches(state: &mut ResponseRendererState, matches: &mut [usize], filter: &str) {
    let response = state.response.to_ref();
    let response_lines = response.lines().collect::<Vec<&str>>();
    let mut lines = state.lines.to_mut();

    matches.iter_mut().for_each(|match_index| {
        if let Some(matching_line) = response_lines.get(*match_index) {
            let mut matched_display_line = lines.get_mut(*match_index);

            if let Some(ref mut display_line_value) = matched_display_line {
                let mut display_line = (*display_line_value).to_mut();
                let mut spans = display_line.spans.to_mut();

                // Reset backgrounds before applying new highlights
                spans.iter_mut().for_each(|span| {
                    let og_bg = *span.to_ref().original_background.to_ref();
                    if let Some(bg) = og_bg {
                        let mut s = span.to_mut();
                        s.original_background.set(None);
                        s.background.set(bg);
                    }
                });

                matching_line.match_indices(filter).for_each(|(index, _)| {
                    let last_ndx = index + filter.len();
                    for span_ndx in index..last_ndx {
                        if let Some(span) = spans.get_mut(span_ndx) {
                            let mut s = span.to_mut();
                            let og_bg = Some(*s.background.to_ref());
                            s.original_background.set(og_bg);
                            s.background.set(Hex::from((255, 255, 0)));
                        }
                    }
                })
            };
        };
    });
}

fn scroll_to_line(
    state: &mut ResponseRendererState,
    elements: &mut Elements<'_, '_>,
    _: Context<'_, ResponseRendererState>,
    line: usize,
) {
    elements
        .by_attribute("id", "container")
        .each(|el, _attributes| {
            let overflow = el.to::<Overflow>();

            state.scroll_position.set(line);

            let pos = Pos {
                x: 0,
                y: line as i32,
            };

            overflow.scroll_to(pos);

            // NOTE: This call to scroll_up_by(0) is to paint the Overflow widget as dirty and
            // scroll to the exact line that I want to scroll to
            overflow.scroll_up_by(0);
        });
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseRendererMessages {
    ResponseUpdate((String, String, Option<String>)),
    FilterUpdate(TextFilter),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TextFilter {
    pub indexes: Vec<usize>,
    pub total: usize,
    pub nav_index: usize,
    pub filter: String,
}
