use core::f32;
use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    components::response_renderer::ResponseRendererMessages,
    options::{get_syntax_theme, get_syntax_themes},
};

use anathema::{
    component::{Component, ComponentId},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{List, State, Value},
};

// TODO: Refactor this selector window to reuse for syntax theme selector, endpoint selector, and
// project window

pub const TEMPLATE: &str = "./src/components/floating_windows/templates/syntax_theme_selector.aml";
const CODE_SAMPLE: &str = include_str!("../../../themes/code_sample.rs");

// TODO: Fix the default project row color to the correct gray
const DEFAULT_ROW_COLOR: &str = "#333333";
const SELECTED_ROW_COLOR: &str = "#FFFFFF";

#[derive(Default, State)]
pub struct SyntaxThemeSelectorState {
    cursor: Value<u8>,
    current_first_index: Value<u8>,
    current_last_index: Value<u8>,
    visible_rows: Value<u8>,
    window_list: Value<List<SyntaxTheme>>,
    count: Value<u8>,
    selected_item: Value<String>,
    code_sample: Value<String>,
    width: Value<f32>,
    height: Value<f32>,
}

impl SyntaxThemeSelectorState {
    pub fn new() -> Self {
        SyntaxThemeSelectorState {
            cursor: 0.into(),
            count: 0.into(),
            current_first_index: 0.into(),
            current_last_index: 4.into(),
            visible_rows: 5.into(),
            window_list: List::empty(),
            selected_item: "".to_string().into(),
            code_sample: String::from(CODE_SAMPLE).into(),
            width: 0f32.into(),
            height: 0f32.into(),
        }
    }
}

#[derive(anathema::state::State)]
struct SyntaxTheme {
    name: Value<String>,
    row_color: Value<String>,
}

impl From<String> for SyntaxTheme {
    fn from(value: String) -> Self {
        SyntaxTheme {
            name: value.replace(".tmTheme", "").into(),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
        }
    }
}

#[derive(Default)]
pub struct SyntaxThemeSelector {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
    items_list: Vec<String>,
}

impl SyntaxThemeSelector {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let id = builder.register_component(
            "syntax_theme_selector",
            TEMPLATE,
            SyntaxThemeSelector::new(ids.clone()),
            SyntaxThemeSelectorState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("syntax_theme_selector"), id);

            new_map
        });

        Ok(())
    }

    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        SyntaxThemeSelector {
            component_ids,
            items_list: vec![],
        }
    }

    fn move_cursor_down(
        &self,
        state: &mut SyntaxThemeSelectorState,
        context: &mut anathema::prelude::Context<'_, SyntaxThemeSelectorState>,
    ) {
        let last_complete_list_index = self.items_list.len().saturating_sub(1);
        let new_cursor = min(*state.cursor.to_ref() + 1, last_complete_list_index as u8);
        state.cursor.set(new_cursor);

        let mut first_index = *state.current_first_index.to_ref();
        let mut last_index = *state.current_last_index.to_ref();

        if new_cursor > last_index {
            last_index = new_cursor;
            first_index = new_cursor - (*state.visible_rows.to_ref() - 1);

            state.current_first_index.set(first_index);
            state.current_last_index.set(last_index);
        }

        self.update_list(
            first_index.into(),
            last_index.into(),
            new_cursor.into(),
            state,
            context,
        );
    }

    fn move_cursor_up(
        &self,
        state: &mut SyntaxThemeSelectorState,
        context: &mut anathema::prelude::Context<'_, SyntaxThemeSelectorState>,
    ) {
        let new_cursor = max(state.cursor.to_ref().saturating_sub(1), 0);
        state.cursor.set(new_cursor);

        let mut first_index = *state.current_first_index.to_ref();
        let mut last_index = *state.current_last_index.to_ref();

        if new_cursor < first_index {
            first_index = new_cursor;
            last_index = new_cursor + (*state.visible_rows.to_ref() - 1);

            state.current_first_index.set(first_index);
            state.current_last_index.set(last_index);
        }

        self.update_list(
            first_index.into(),
            last_index.into(),
            new_cursor.into(),
            state,
            context,
        );
    }

    fn update_list(
        &self,
        first_index: usize,
        last_index: usize,
        selected_index: usize,
        state: &mut SyntaxThemeSelectorState,
        context: &mut anathema::prelude::Context<'_, SyntaxThemeSelectorState>,
    ) {
        let display_items = &self.items_list[first_index..=last_index];
        let mut new_items_list: Vec<SyntaxTheme> = vec![];
        display_items.iter().for_each(|syntax_theme| {
            new_items_list.push((*syntax_theme).to_string().into());
        });

        loop {
            if state.window_list.len() > 0 {
                state.window_list.pop_front();
            } else {
                break;
            }
        }

        let mut theme_name: String = String::new();
        let mut new_list_state = List::<SyntaxTheme>::empty();
        new_items_list
            .into_iter()
            .enumerate()
            .for_each(|(index, mut syntax_theme)| {
                let visible_index = selected_index.saturating_sub(first_index);
                if index == visible_index {
                    syntax_theme.row_color = SELECTED_ROW_COLOR.to_string().into();
                    theme_name = syntax_theme.name.to_ref().to_string();
                } else {
                    syntax_theme.row_color = DEFAULT_ROW_COLOR.to_string().into();
                }

                new_list_state.push(syntax_theme);
            });

        self.update_code_sample(context, &theme_name);

        state.window_list = new_list_state;
    }

    fn update_code_sample(
        &self,
        context: &mut anathema::prelude::Context<'_, SyntaxThemeSelectorState>,
        theme_name: &str,
    ) {
        let component_ids = self.component_ids.try_borrow();
        if component_ids.is_err() {
            return;
        }

        let component_ids = component_ids.unwrap();
        let code_sample_id = component_ids.get("code_sample_renderer");
        if code_sample_id.is_none() {
            return;
        }

        let code_sample_id = code_sample_id.unwrap();

        let code = String::from(CODE_SAMPLE);
        let ext = String::from("rs");
        if let Ok(msg) = serde_json::to_string(&ResponseRendererMessages::ResponseUpdate((
            code,
            ext,
            Some(theme_name.to_string()),
        ))) {
            context.emit(*code_sample_id, msg);
        }
    }

    fn resize_window(
        &self,
        state: &mut SyntaxThemeSelectorState,
        context: &mut anathema::prelude::Context<'_, SyntaxThemeSelectorState>,
    ) {
        let viewport_size = context.viewport.size();
        let vp_width = viewport_size.width as f32;
        let vp_height = viewport_size.height as f32;
        let desired_width = f32::ceil(vp_width * 0.7);
        let desired_height = f32::ceil(vp_height * 0.7);

        let visible_rows = (desired_height - 2f32) as u8;

        state.width.set(desired_width);
        state.height.set(desired_height);
        state.visible_rows.set(visible_rows);
    }
}

impl Component for SyntaxThemeSelector {
    type State = SyntaxThemeSelectorState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn resize(
        &mut self,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        // NOTE: Causes a panic in anathema, revisit after updating anathema fork
        //
        // self.resize_window(state, &mut context);

        // let current_last_index =
        //     min(*state.visible_rows.to_ref(), self.items_list.len() as u8).saturating_sub(1);
        // state.cursor.set(0);
        // state.current_first_index.set(0);
        // state.current_last_index.set(current_last_index);

        // let first_index: usize = *state.current_first_index.to_ref() as usize;
        // let last_index: usize = *state.current_last_index.to_ref() as usize;
        //
        // self.update_list(first_index, last_index, selected_index, state, &mut context);
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let current_syntax_theme = get_syntax_theme();

        state.selected_item.set(
            current_syntax_theme
                .replace("themes/", "")
                .replace(".tmTheme", ""),
        );

        self.items_list = get_syntax_themes();

        self.resize_window(state, &mut context);

        let current_last_index =
            min(*state.visible_rows.to_ref(), self.items_list.len() as u8).saturating_sub(1);
        state.cursor.set(0);
        state.current_first_index.set(0);
        state.current_last_index.set(current_last_index);

        let first_index: usize = *state.current_first_index.to_ref() as usize;
        let last_index: usize = *state.current_last_index.to_ref() as usize;
        let selected_index = 0;

        self.update_list(first_index, last_index, selected_index, state, &mut context);
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match event.code {
            anathema::component::KeyCode::Char(char) => match char {
                'j' => self.move_cursor_down(state, &mut context),
                'k' => self.move_cursor_up(state, &mut context),
                _ => {}
            },

            anathema::component::KeyCode::Up => self.move_cursor_up(state, &mut context),
            anathema::component::KeyCode::Down => self.move_cursor_down(state, &mut context),

            anathema::component::KeyCode::Esc => {
                // NOTE: This sends cursor to satisfy publish() but is not used
                context.publish("syntax_theme_selector__cancel", |state| &state.cursor)
            }

            anathema::component::KeyCode::Enter => {
                let selected_index = *state.cursor.to_ref() as usize;
                let theme = self.items_list.get(selected_index);

                match theme {
                    Some(theme) => {
                        // NOTE: Clean up the .tmTheme string replace here
                        state
                            .selected_item
                            .set(theme.to_string().replace(".tmTheme", ""));
                        context.publish("syntax_theme_selector__selection", |state| {
                            &state.selected_item
                        });
                    }
                    None => context.publish("syntax_theme_selector__cancel", |state| &state.cursor),
                }
            }

            _ => {}
        }
    }
}
