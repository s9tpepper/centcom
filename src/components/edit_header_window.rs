use anathema::{
    component::{self, Component, KeyCode},
    state::{State, Value},
};

pub const EDIT_HEADER_WINDOW_TEMPLATE: &str = "./src/components/templates/edit_header_window.aml";

#[derive(Default)]
pub struct EditHeaderWindow;

#[derive(Default, State)]
pub struct EditHeaderWindowState {
    name: Value<String>,
    value: Value<String>,
}

impl EditHeaderWindowState {
    pub fn new() -> Self {
        EditHeaderWindowState {
            name: "".to_string().into(),
            value: "".to_string().into(),
        }
    }
}

impl Component for EditHeaderWindow {
    type State = EditHeaderWindowState;
    type Message = String;

    fn receive(
        &mut self,
        ident: &str,
        value: anathema::state::CommonVal<'_>,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match ident {
            "edit_header_name_update" => {
                state.name.set(value.to_string());

                context.publish("edit_header_name_update", |state| &state.name)
            }

            "edit_header_value_update" => {
                state.value.set(value.to_string());

                context.publish("edit_header_value_update", |state| &state.value)
            }
            _ => {}
        }
    }

    fn on_key(
        &mut self,
        key: component::KeyEvent,
        _state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match key.code {
            KeyCode::Esc => {
                context.publish("cancel_edit_header", |state| &state.name);
            }

            KeyCode::Char(char) => {
                match char {
                    's' => context.publish("edit_header", |state| &state.name),

                    // Sets focus to header name text input
                    'n' => context.set_focus("id", "edit_header_name_input_id"),

                    // Sets focus to header value text input
                    'v' => context.set_focus("id", "edit_header_value_input_id"),

                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
