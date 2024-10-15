use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::messages::confirm_delete_project::{ComponentMessage, ConfirmDeleteProject};

pub const CONFIRM_ACTION_WINDOW_TEMPLATE: &str =
    "./src/components/templates/confirm_action_window.aml";

#[derive(Default)]
pub struct ConfirmActionWindow;

impl ConfirmActionWindow {
    pub fn new() -> Self {
        ConfirmActionWindow {}
    }
}

#[derive(Default, State)]
pub struct ConfirmActionWindowState {
    title: Value<String>,
    message: Value<String>,
}

impl ConfirmActionWindowState {
    pub fn new() -> Self {
        ConfirmActionWindowState {
            title: "".to_string().into(),
            message: "".to_string().into(),
        }
    }
}

impl Component for ConfirmActionWindow {
    type State = ConfirmActionWindowState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        false
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        if let Ok(component_message) =
            serde_json::from_str::<ConfirmDeleteProject>(message.as_str())
        {
            let message_type = component_message.get_message_type();
            if message_type.as_str() == "confirm_delete_project" {
                state.title.set(component_message.title);
                state.message.set(component_message.message);
            }
        }
    }
}
