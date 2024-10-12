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

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut elements: component::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        println!("I got a message!: {message}");
    }

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
                if char == 's' {
                    context.publish("edit_header", |state| &state.name)
                }
            }

            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
