use anathema::{
    component::{self, Component, KeyCode},
    state::{State, Value},
};

use super::request_headers_editor::Header;

pub const ADD_HEADER_WINDOW_TEMPLATE: &str = "./src/components/templates/add_header_window.aml";

#[derive(Default)]
pub struct AddHeaderWindow;

#[derive(Default, State)]
pub struct AddHeaderWindowState {
    header: Value<Header>,
    hello: Value<String>,
}

impl AddHeaderWindowState {
    pub fn new() -> Self {
        AddHeaderWindowState {
            hello: "world".to_string().into(),
            header: Header {
                name: "".to_string().into(),
                value: "".to_string().into(),
            }
            .into(),
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
        _context: anathema::prelude::Context<'_, Self::State>,
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
            "header_name_update" => state.header.to_mut().name.set(value.to_string()),
            "header_value_update" => state.header.to_mut().value.set(value.to_string()),
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
                // panic!("at the disco!");
                // println!("Publishing cancel_add_header");
                context.publish("cancel_add_header", |state| &state.header);
                println!("Published cancel_add_header");
            }

            KeyCode::Char(char) => match char {
                's' => context.publish("add_header", |state| &state.header),
                'c' => {}
                _ => {}
            },

            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
