use std::{cell::Ref, collections::HashMap};

use anathema::{
    component::{self, Component, ComponentId, KeyCode},
    state::{State, Value},
};

use super::{
    dashboard::{DashboardMessageHandler, FloatingWindow},
    request_headers_editor::HeaderState,
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

impl DashboardMessageHandler for AddHeaderWindow {
    fn handle_message(
        value: component::CommonVal<'_>,
        ident: impl Into<String>,
        state: &mut super::dashboard::DashboardState,
        mut context: anathema::prelude::Context<'_, super::dashboard::DashboardState>,
        _component_ids: Ref<'_, HashMap<String, ComponentId<String>>>,
    ) {
        let event: String = ident.into();
        match event.as_str() {
            "add_header__name_update" => state.new_header_name.set(value.to_string()),
            "add_header__value_update" => state.new_header_value.set(value.to_string()),
            "add_header__submit" => {
                let header_name = state.new_header_name.to_ref().to_string();
                let header_value = state.new_header_value.to_ref().to_string();

                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");

                if header_name.trim().is_empty() || header_value.trim().is_empty() {
                    return;
                }

                let header = HeaderState {
                    name: header_name.into(),
                    value: header_value.into(),
                };
                state.request_headers.push(header);
            }
            "add_header__cancel" => {
                state.floating_window.set(FloatingWindow::None);
                state.new_header_name.set("".to_string());
                state.new_header_value.set("".to_string());
                context.set_focus("id", "app");
            }

            _ => {}
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
            // "name_input_focus" => match value.to_bool() {
            //     true => {}
            //     false => {} // false => context.set_focus("id", "add_header_window"),
            // },
            //
            // "value_input_focus" => match value.to_bool() {
            //     true => {}
            //     false => {} // false => context.set_focus("id", "add_header_window"),
            // },
            "header_name_update" => {
                state.name.set(value.to_string());

                context.publish("add_header__name_update", |state| &state.name)
            }

            "header_value_update" => {
                state.value.set(value.to_string());

                context.publish("add_header__value_update", |state| &state.value)
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
                context.publish("add_header__cancel", |state| &state.name);
            }

            KeyCode::Char(char) => {
                match char {
                    's' => context.publish("add_header__submit", |state| &state.name),

                    // Sets focus to header name text input
                    'n' => context.set_focus("id", "header_name_input"),

                    // Sets focus to header value text input
                    'v' => context.set_focus("id", "header_value_input"),

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
