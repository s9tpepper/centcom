use anathema::{
    component::Component,
    state::{State, Value},
};

pub const EDIT_HEADER_SELECTOR_TEMPLATE: &str =
    "./src/components/templates/edit_header_selector.aml";

#[derive(Default)]
pub struct EditHeaderSelector;

#[derive(Default, State)]
pub struct EditHeaderSelectorState {
    selection: Value<Option<char>>,
}

impl EditHeaderSelectorState {
    pub fn new() -> Self {
        EditHeaderSelectorState {
            selection: None.into(),
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
                    context.publish("header_selection", |state| &state.selection)
                }
            }

            anathema::component::KeyCode::Esc => {
                // NOTE: This selection state needs a Some in order for the associated function to
                // fire
                state.selection.set(Some('x'));
                context.publish("cancel_header_selection", |state| &state.selection)
            }

            _ => {}
        }
    }
}
