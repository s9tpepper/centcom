use anathema::{
    component::Component,
    state::{State, Value},
    widgets::Elements,
};

use crate::theme::{get_app_theme, AppTheme};

use super::dashboard::{DashboardMessageHandler, FloatingWindow};

pub const EDIT_HEADER_SELECTOR_TEMPLATE: &str =
    "./src/components/templates/edit_header_selector.aml";

#[derive(Default)]
pub struct EditHeaderSelector;

#[derive(Default, State)]
pub struct EditHeaderSelectorState {
    selection: Value<Option<char>>,
    app_theme: Value<AppTheme>,
}

impl EditHeaderSelectorState {
    pub fn new() -> Self {
        let app_theme = get_app_theme();

        EditHeaderSelectorState {
            selection: None.into(),
            app_theme: app_theme.into(),
        }
    }
}

impl DashboardMessageHandler for EditHeaderSelector {
    fn handle_message(
        value: anathema::state::CommonVal<'_>,
        ident: impl Into<String>,
        state: &mut super::dashboard::DashboardState,
        mut context: anathema::prelude::Context<'_, super::dashboard::DashboardState>,
        _: Elements<'_, '_>,
        component_ids: std::cell::Ref<
            '_,
            std::collections::HashMap<String, anathema::component::ComponentId<String>>,
        >,
    ) {
        let event: String = ident.into();

        match event.as_str() {
            "edit_header_selector__cancel" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");
            }

            "edit_header_selector__selection" => {
                let selection: usize = value.to_string().parse().unwrap();
                let mut endpoint = state.endpoint.to_mut();

                let last_index = endpoint.headers.len().saturating_sub(1);
                if selection > last_index {
                    return;
                }

                let header = endpoint.headers.remove(selection);

                if let Some(selected_header) = &header {
                    let header = selected_header.to_ref();
                    state.edit_header_name.set(header.name.to_ref().clone());
                    state.edit_header_value.set(header.value.to_ref().clone());
                };

                state.header_being_edited.set(header);
                state.floating_window.set(FloatingWindow::EditHeader);

                let edit_header_name_input_id = component_ids.get("edit_header_name_input");
                if let Some(id) = edit_header_name_input_id {
                    context.emit(*id, state.edit_header_name.to_ref().clone());
                }

                let edit_header_value_input_id = component_ids.get("edit_header_value_input");
                if let Some(id) = edit_header_value_input_id {
                    context.emit(*id, state.edit_header_value.to_ref().clone());
                }

                context.set_focus("id", "edit_header_window");
            }

            _ => {}
        }
    }
}

impl Component for EditHeaderSelector {
    type State = EditHeaderSelectorState;
    type Message = ();

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_key(
        &mut self,
        event: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match event.code {
            anathema::component::KeyCode::Char(char) => {
                state.selection.set(Some(char));
                if let '0'..='9' = char {
                    context.publish("edit_header_selector__selection", |state| &state.selection)
                }
            }

            anathema::component::KeyCode::Esc => {
                // NOTE: This selection state needs a Some in order for the associated function to
                // fire
                state.selection.set(Some('x'));
                context.publish("edit_header_selector__cancel", |state| &state.selection)
            }

            _ => {}
        }
    }
}
