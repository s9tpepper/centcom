use anathema::{
    component::{self, Component, KeyCode},
    state::{State, Value},
};

pub const ADD_HEADER_WINDOW_TEMPLATE: &str = "./src/components/templates/add_header_window.aml";

#[derive(Default)]
pub struct AddHeaderWindow;

#[derive(Default, State)]
pub struct AddHeaderWindowState {
    name: Value<String>,
    value: Value<String>,
}

impl AddHeaderWindowState {
    pub fn new() -> Self {
        AddHeaderWindowState {
            name: "".to_string().into(),
            value: "".to_string().into(),
        }
    }
}

impl Component for AddHeaderWindow {
    type State = AddHeaderWindowState;
    type Message = ();

    fn receive(
        &mut self,
        ident: &str,
        value: anathema::state::CommonVal<'_>,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match ident {
            "name_input_focus" => match value.to_bool() {
                true => {}
                false => {} // false => context.set_focus("id", "add_header_window"),
            },

            "value_input_focus" => match value.to_bool() {
                true => {}
                false => {} // false => context.set_focus("id", "add_header_window"),
            },

            "header_name_update" => {
                state.name.set(value.to_string());

                context.publish("header_name_update", |state| &state.name)
            }

            "header_value_update" => {
                state.value.set(value.to_string());

                context.publish("header_value_update", |state| &state.value)
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
                context.publish("cancel_add_header", |state| &state.name);
            }

            KeyCode::Char(char) => {
                if char == 's' {
                    context.publish("add_header", |state| &state.name)
                }
            }

            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
