use std::{
    cell::RefCell,
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
};

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

use crate::{
    options::get_syntax_theme,
    theme::{get_app_theme, get_app_theme_persisted, AppTheme},
};

use super::syntax_highlighter::{highlight, Instruction, Parser};

const TEMPLATE: &str = "./src/components/templates/response_renderer.aml";
pub const CODE_SAMPLE: &str = include_str!("../../themes/code_sample.rs");

enum ScrollDirection {
    Up,
    Down,
}

pub struct ResponseRenderer {
    #[allow(unused)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
    syntax_highlighter_cursor: Pos,
    foreground: Hex,
    background: Hex,
    text_filter: TextFilter,
    theme: Option<Theme>,

    // overflow: Option<&'app mut Overflow>,
    size: Option<Size>,
    response_reader: Option<BufReader<File>>,
    response_offset: usize,
    viewport_height: usize,
    extension: String,

    // All lines from the response
    response_lines: Vec<String>,

    code_sample: Option<String>,
    code_ext: Option<String>,
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

        let mut ids_ref = ids.borrow_mut();
        ids_ref.insert(ident, id);

        Ok(())
    }

    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        ResponseRenderer {
            component_ids,
            syntax_highlighter_cursor: Pos::ZERO,
            foreground: Hex::from((255, 255, 255)),
            background: Hex::BLACK,
            text_filter: TextFilter {
                ..Default::default()
            },
            theme: None,
            response_reader: None,
            response_offset: 0,
            viewport_height: 0,
            size: None,
            extension: "".to_string(),
            response_lines: vec![],
            code_ext: None,
            code_sample: None,
        }
    }

    fn update_app_theme(&self, state: &mut ResponseRendererState) {
        let app_theme = get_app_theme_persisted();

        // state.app_theme.set(app_theme);
        // TODO: Temp fix for weirdness around state updates to the app_theme
        let mut at = state.app_theme.to_mut();
        at.background.set(app_theme.background);
        at.foreground.set(app_theme.foreground);
        at.project_name_background
            .set(app_theme.project_name_background);
        at.project_name_foreground
            .set(app_theme.project_name_foreground);
        at.border_focused.set(app_theme.border_focused);
        at.border_unfocused.set(app_theme.border_unfocused);
        at.overlay_heading.set(app_theme.overlay_heading);
        at.overlay_background.set(app_theme.overlay_background);
        at.overlay_foreground.set(app_theme.overlay_foreground);
        at.overlay_submit_background
            .set(app_theme.overlay_submit_background);
        at.overlay_submit_foreground
            .set(app_theme.overlay_submit_foreground);

        at.overlay_cancel_background
            .set(app_theme.overlay_cancel_background);
        at.overlay_cancel_foreground
            .set(app_theme.overlay_cancel_foreground);
        at.menu_color_1.set(app_theme.menu_color_1);
        at.menu_color_2.set(app_theme.menu_color_2);
        at.menu_color_3.set(app_theme.menu_color_3);
        at.menu_color_4.set(app_theme.menu_color_4);
        at.menu_color_5.set(app_theme.menu_color_5);

        at.endpoint_name_background
            .set(app_theme.endpoint_name_background);
        at.endpoint_name_foreground
            .set(app_theme.endpoint_name_foreground);
        at.menu_opt_background.set(app_theme.menu_opt_background);
        at.menu_opt_foreground.set(app_theme.menu_opt_foreground);
        at.top_bar_background.set(app_theme.top_bar_background);
        at.top_bar_foreground.set(app_theme.top_bar_foreground);
        at.bottom_bar_background
            .set(app_theme.bottom_bar_background);
        at.bottom_bar_foreground
            .set(app_theme.bottom_bar_foreground);
    }

    // TODO: Fix update_cursor/update_cursor2 so I only need 2
    fn update_cursor2(
        &mut self,
        state: &mut ResponseRendererState,
        elements: &mut Elements<'_, '_>,
    ) {
        if self.size.is_none() {
            return;
        }

        elements
            .by_attribute("id", "container")
            .first(|element, _| {
                let overflow = element.to::<Overflow>();

                let size: Size = self.size.unwrap();

                // Make sure there are enough lines and spans
                while self.syntax_highlighter_cursor.y as usize >= state.lines.len() {
                    state.lines.push_back(Line::empty());
                }

                {
                    let mut lines = state.lines.to_mut();
                    let line = lines
                        .get_mut(self.syntax_highlighter_cursor.y as usize)
                        .unwrap();

                    let spans = &mut line.to_mut().spans;
                    while self.syntax_highlighter_cursor.x as usize > spans.len() {
                        spans.push_back(Span::empty());
                    }
                }

                let mut screen_cursor = self.syntax_highlighter_cursor - overflow.offset();

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
                state.buf_cursor_x.set(self.syntax_highlighter_cursor.x);
                state.buf_cursor_y.set(self.syntax_highlighter_cursor.y);
            });
    }

    fn update_cursor(
        &mut self,
        state: &mut ResponseRendererState,
        overflow: &mut Overflow,
        size: Size,
    ) {
        // Make sure there are enough lines and spans
        while self.syntax_highlighter_cursor.y as usize >= state.lines.len() {
            state.lines.push_back(Line::empty());
        }

        {
            let mut lines = state.lines.to_mut();
            let line = lines
                .get_mut(self.syntax_highlighter_cursor.y as usize)
                .unwrap();

            let spans = &mut line.to_mut().spans;
            while self.syntax_highlighter_cursor.x as usize > spans.len() {
                spans.push_back(Span::empty());
            }
        }

        let mut screen_cursor = self.syntax_highlighter_cursor - overflow.offset();

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
        state.buf_cursor_x.set(self.syntax_highlighter_cursor.x);
        state.buf_cursor_y.set(self.syntax_highlighter_cursor.y);
    }

    // TODO: Fix apply_inst2/apply_inst so I only need apply_inst2
    pub fn apply_inst2(
        &mut self,
        inst: &Instruction,
        state: &mut ResponseRendererState,
        elements: &mut Elements<'_, '_>,
    ) {
        state.current_instruction.set(Some(format!("{inst:?}")));

        if self.size.is_none() {
            return;
        }

        match inst {
            Instruction::Type(c, bold) => {
                let mut added_char = false;

                {
                    let mut lines = state.lines.to_mut();
                    let line = lines.get_mut(self.syntax_highlighter_cursor.y as usize);

                    if line.is_none() {
                        return;
                    }

                    let line = line.unwrap();
                    let mut_line = RefCell::new(line.to_mut());

                    {
                        let mut mutable_line = mut_line.try_borrow_mut().unwrap();
                        let spans_len = mutable_line.spans.len();

                        let mut spans = mutable_line.spans.to_mut();
                        let mut previous_span = spans.get_mut(spans_len);

                        if let Some(ref mut prev_span) = previous_span {
                            let mut previous = prev_span.to_mut();
                            if *previous.bold.to_ref() == *bold
                                && *previous.background.to_ref() == self.background
                                && *previous.foreground.to_ref() == self.foreground
                            {
                                let char_index = previous.text.to_ref().len();
                                previous.text.to_mut().insert(char_index, *c);

                                added_char = true;
                            }
                        }
                    }

                    {
                        if !added_char {
                            let mut mutable_line = mut_line.try_borrow_mut().unwrap();
                            mutable_line.spans.insert(
                                self.syntax_highlighter_cursor.x as usize,
                                Span::new(c.to_string(), self.foreground, self.background, *bold),
                            );

                            added_char = true;
                        }
                    }
                }

                if added_char {
                    self.syntax_highlighter_cursor.x += 1;
                    self.update_cursor2(state, elements);
                }
            }
            Instruction::SetForeground(hex) => self.foreground = *hex,
            Instruction::SetBackground(hex) => self.background = *hex,
            Instruction::Newline { x } => {
                self.syntax_highlighter_cursor.x = *x;
                self.syntax_highlighter_cursor.y += 1;
                self.update_cursor2(state, elements);
            }
            Instruction::SetX(x) => {
                self.syntax_highlighter_cursor.x = *x;
                self.update_cursor2(state, elements);
            }
        }
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
                Instruction::Type(c, bold) => {
                    {
                        let mut lines = state.lines.to_mut();
                        let line = lines.get_mut(self.syntax_highlighter_cursor.y as usize);

                        if line.is_none() {
                            return;
                        }

                        let line = line.unwrap();
                        let mut line = line.to_mut();
                        // let spans = line.spans.len();

                        line.spans.insert(
                            self.syntax_highlighter_cursor.x as usize,
                            Span::new(c.to_string(), self.foreground, self.background, *bold),
                        );
                        self.syntax_highlighter_cursor.x += 1;
                    }

                    self.update_cursor(state, vp, size);
                }
                Instruction::SetForeground(hex) => self.foreground = *hex,
                Instruction::SetBackground(hex) => self.background = *hex,
                Instruction::Newline { x } => {
                    self.syntax_highlighter_cursor.x = *x;
                    self.syntax_highlighter_cursor.y += 1;
                    self.update_cursor(state, vp, size);
                }
                Instruction::SetX(x) => {
                    self.syntax_highlighter_cursor.x = *x;
                    self.update_cursor(state, vp, size);
                }
            }
        });
    }

    // NOTE: Renders initial response when its new
    // TODO: Make this work for scrolling the response
    fn render_response(
        &mut self,
        extension: String,
        elements: &mut Elements<'_, '_>,
        state: &mut ResponseRendererState,
        offset: usize,
    ) {
        if self.response_reader.is_none() {
            return;
        }

        if self.size.is_none() {
            return;
        }

        self.extension = extension;

        let size = self.size.unwrap();
        let response_reader = self.response_reader.as_mut().unwrap();
        self.response_offset = offset;
        self.viewport_height = size.height;

        let mut buf: Vec<u8> = vec![];
        match response_reader.read_to_end(&mut buf) {
            Ok(_) => {
                let response = String::from_utf8(buf).unwrap_or(String::from("oops"));
                let lines = response.lines();
                let response_lines: Vec<String> = lines.map(|s| s.to_string()).collect();
                self.response_lines = response_lines;
            }
            // TODO: Figure out what to do if this fails
            Err(_) => todo!(),
        }

        self.scroll_response(elements, state, offset);
    }

    fn scroll_response(
        &mut self,
        elements: &mut Elements<'_, '_>,
        state: &mut ResponseRendererState,
        offset: usize,
    ) {
        if self.response_reader.is_none() {
            return;
        }

        if self.size.is_none() {
            return;
        }

        let size = self.size.unwrap();
        self.response_offset = offset;
        self.viewport_height = size.height;

        let mut viewable_lines: Vec<String> = vec![];

        let last_response_line_index = self.response_lines.len();
        let last_viewable_index = self.response_offset + self.viewport_height;
        let ending_index = min(last_viewable_index, last_response_line_index);

        for index in self.response_offset..ending_index {
            let line = &self.response_lines[index];

            if line.len() > size.width {
                let (new_line, _) = line.split_at(size.width.saturating_sub(5));

                let t = format!("{new_line}...");

                viewable_lines.push(t);
            } else {
                viewable_lines.push(line.to_string());
            }
        }

        let theme = get_syntax_theme();
        let viewable_response = viewable_lines.join("\n");

        let screens = last_response_line_index as f32 / self.viewport_height as f32;
        let current_screen = self.response_offset as f32 / self.viewport_height as f32;
        let percent = (current_screen / screens) * 100f32;
        let percent_scrolled = format!("{:0>2}", percent as usize);

        state.percent_scrolled.set(percent_scrolled);

        self.set_response(state, viewable_response, Some(theme), elements);
    }

    // NOTE: This one is now the one setting the response in the response text area with the syntax
    // highlighting
    fn set_response(
        &mut self,
        state: &mut ResponseRendererState,
        response: String,
        theme: Option<String>,
        elements: &mut Elements<'_, '_>,
    ) {
        loop {
            if state.lines.len() == 0 {
                break;
            }

            state.lines.remove(0);
        }

        let (highlighted_lines, parsed_theme) = highlight(&response, &self.extension, theme);

        let bg = parsed_theme.settings.background;
        if self.theme.is_none() {
            self.theme = Some(parsed_theme);
        }

        if let Some(color) = bg {
            let hex_color = format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b);
            state.response_background.set(hex_color);
        }

        let instructions = Parser::new(highlighted_lines).instructions();
        for instruction in instructions {
            self.apply_inst2(&instruction, state, elements);
        }

        state.response.set(response);
    }

    fn update_size(&mut self, context: Context<'_, ResponseRendererState>) {
        let size = context.viewport.size();

        let app_titles = 2; // top/bottom menus of dashboard
        let url_method_inputs = 3; // height of url and method inputs with borders
        let response_borders = 2; // borders around response input
        let total_height_offset = app_titles + url_method_inputs + response_borders;

        self.size = Some(Size {
            width: size.width,
            height: size.height - total_height_offset,
        });
    }

    fn scroll(
        &mut self,
        state: &mut ResponseRendererState,
        mut elements: Elements<'_, '_>,
        _: Context<'_, ResponseRendererState>,
        direction: ScrollDirection,
    ) {
        let new_offset = match direction {
            ScrollDirection::Up => self.response_offset.saturating_sub(self.viewport_height),
            ScrollDirection::Down => self.response_offset + self.viewport_height,
        };

        self.scroll_response(&mut elements, state, new_offset);
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
    text: Value<String>,
    bold: Value<bool>,
    foreground: Value<Hex>,
    background: Value<Hex>,
    original_background: Value<Option<Hex>>,
}

impl Span {
    pub fn new(c: String, foreground: Hex, background: Hex, bold: bool) -> Self {
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
            text: " ".to_string().into(),
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
    // Rendered lines in the text area for current page
    pub lines: Value<List<Line>>,
    pub current_instruction: Value<Option<String>>,
    pub title: Value<String>,
    pub waiting: Value<String>,
    pub show_cursor: Value<bool>,
    pub response: Value<String>,
    pub response_background: Value<String>,
    pub percent_scrolled: Value<String>,
    pub app_theme: Value<AppTheme>,
}

impl ResponseRendererState {
    pub fn new() -> Self {
        let app_theme = get_app_theme();

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
            percent_scrolled: "0".to_string().into(),
            app_theme: app_theme.into(),
        }
    }
}

impl Component for ResponseRenderer {
    type State = ResponseRendererState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_focus(
        &mut self,
        _: &mut Self::State,
        _: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        self.update_size(context);
    }

    fn resize(
        &mut self,
        _: &mut Self::State,
        _: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        self.update_size(context);

        // TODO: Update response text when the window gets resized
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
                            'd' => self.scroll(state, elements, context, ScrollDirection::Down),
                            'u' => self.scroll(state, elements, context, ScrollDirection::Up),

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
                ResponseRendererMessages::ThemeUpdate => {
                    self.update_app_theme(state);
                }

                ResponseRendererMessages::ResponseUpdate(extension) => {
                    // TODO: Try to delete this file if the program closes/quits/crashes
                    let reader_result = get_file_reader("/tmp/centcom_response.txt");
                    if reader_result.is_err() {
                        println!("Error getting reader for response...");
                        return;
                    }

                    let response_reader = reader_result.unwrap();
                    self.response_reader = Some(response_reader);
                    self.render_response(extension, &mut elements, state, 0);
                }

                ResponseRendererMessages::SyntaxPreview(theme) => {
                    loop {
                        if state.lines.len() == 0 {
                            break;
                        }

                        state.lines.remove(0);
                    }

                    if self.code_sample.is_none() {
                        self.code_sample = Some(String::from(CODE_SAMPLE));
                        self.code_ext = Some(String::from("rs"));
                    }

                    let code = self.code_sample.clone().unwrap();
                    let ext = self.code_ext.as_ref().unwrap();
                    let (highlighted_lines, parsed_theme) = highlight(&code, ext, theme);

                    // NOTE: Maybe remove this if its not useful
                    if self.theme.is_none() {
                        self.theme = Some(parsed_theme.clone());
                    }

                    if let Some(color) = parsed_theme.settings.background {
                        let hex_color = format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b);
                        state.response_background.set(hex_color);
                    }

                    let instructions = Parser::new(highlighted_lines).instructions();
                    for instruction in instructions {
                        self.apply_inst(&instruction, state, &mut elements);
                    }

                    state.response.set(code.to_string());
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

fn clear_highlights(state: &mut ResponseRendererState) {
    let mut lines = state.lines.to_mut();

    lines.iter_mut().for_each(|line| {
        let mut l = line.to_mut();
        let mut spans = l.spans.to_mut();
        spans.iter_mut().for_each(|span| {
            let mut s = span.to_mut();
            let og_opt = *s.original_background.to_ref();
            if let Some(og_bg) = og_opt {
                s.background.set(og_bg);
                s.original_background.set(None);
            };
        });
    });
}

fn highlight_matches(state: &mut ResponseRendererState, matches: &mut [usize], filter: &str) {
    clear_highlights(state);

    let response = state.response.to_ref();
    let response_lines = response.lines().collect::<Vec<&str>>();
    let mut lines = state.lines.to_mut();

    matches.iter_mut().for_each(|match_index| {
        if let Some(matching_line) = response_lines.get(*match_index) {
            let mut matched_display_line = lines.get_mut(*match_index);

            if let Some(ref mut display_line_value) = matched_display_line {
                let mut display_line = (*display_line_value).to_mut();
                let mut spans = display_line.spans.to_mut();

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

fn get_file_reader(file_path: &str) -> anyhow::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseRendererMessages {
    ResponseUpdate(String),
    SyntaxPreview(Option<String>),
    FilterUpdate(TextFilter),
    ThemeUpdate,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TextFilter {
    pub indexes: Vec<usize>,
    pub total: usize,
    pub nav_index: usize,
    pub filter: String,
}
