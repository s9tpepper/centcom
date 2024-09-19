use std::{fmt::Display, str::FromStr};

use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

pub const METHOD_SELECTOR_TEMPLATE: &str = "./src/components/templates/method_selector.aml";

#[derive(Default)]
pub struct MethodSelector;

#[derive(Default, State)]
pub struct MethodSelectorState {
    selection: Value<String>,
}

impl MethodSelectorState {
    pub fn new() -> Self {
        MethodSelectorState {
            selection: "GET".to_string().into(),
        }
    }
}

impl Component for MethodSelector {
    type State = MethodSelectorState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        // TODO: Highlight current selection
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match event.code {
            KeyCode::Char(char) => {
                match char.to_uppercase().to_string().as_ref() {
                    "G" => state.selection.set("GET".to_string()),
                    "P" => state.selection.set("POST".to_string()),
                    "U" => state.selection.set("PUT".to_string()),
                    "D" => state.selection.set("DELETE".to_string()),
                    "H" => state.selection.set("HEAD".to_string()),
                    "O" => state.selection.set("OPTIONS".to_string()),
                    "C" => state.selection.set("CUSTOM".to_string()),
                    _ => {
                        // NOTE: Prevents other keys from closing the window
                        return;
                    }
                };

                context.publish("new_method_selection", |state| &state.selection);
                context.publish("cancel_method_selector", |state| &state.selection);
                context.set_focus("id", "app")
            }

            KeyCode::Esc => {
                context.publish("cancel_method_selector", |state| &state.selection);
                context.set_focus("id", "app")
            }

            _ => (),
        };
    }
}

#[derive(Default)]
enum Method {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Connect,
    Custom,
}

#[derive(Debug, PartialEq, Eq)]
struct MethodParseError;

impl FromStr for Method {
    type Err = MethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "PATCH" => Ok(Method::Patch),
            "OPTIONS" => Ok(Method::Options),
            "CONNECT" => Ok(Method::Connect),
            _ => Ok(Method::Custom),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Patch => write!(f, "PATCH"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Custom => write!(f, "CUSTOM"),
        }
    }
}
