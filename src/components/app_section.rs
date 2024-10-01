use anathema::{
    component::Component,
    state::{CommonVal, State, Value},
};

pub const APP_SECTION_TEMPLATE: &str = "./src/components/templates/app_section.aml";

#[derive(Default)]
pub struct AppSection;

#[derive(Default, State)]
pub struct AppSectionState {
    section_id: Value<Option<String>>,
    section_text_id: Value<Option<String>>,
}

impl AppSectionState {
    pub fn new() -> Self {
        AppSectionState {
            section_id: None.into(),
            section_text_id: None.into(),
        }
    }
}

impl Component for AppSection {
    type State = AppSectionState;
    type Message = ();

    fn tick(
        &mut self,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
        _dt: std::time::Duration,
    ) {
        if state.section_id.to_ref().is_none() {
            let Some(section_id) = context.get_external("section_id") else {
                return;
            };

            let Some(section_text_id) = context.get_external("section_text_id") else {
                return;
            };

            if let Some(id) = section_id.to_common() {
                let id = id.to_string();
                state.section_id.set(Some(id));
            }

            if let Some(section_text_id) = section_text_id.to_common() {
                let text_id = section_text_id.to_string();
                state.section_text_id.set(Some(text_id));
            }
        }
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        let section_id = state.section_id.to_ref().clone();
        if let Some(section_id) = section_id {
            let section_id = section_id.to_string().leak();
            elements
                .by_attribute("id", CommonVal::Str(section_id))
                .each(|_element, attributes| {
                    attributes.set("foreground", "#ffffff");
                })
        }

        let section_text_id = state.section_text_id.to_ref().clone();
        if let Some(section_text_id) = section_text_id {
            // NOTE: Is this right way to do this? Not so sure about this .leak()
            let id = section_text_id.to_string().leak();
            context.set_focus("id", CommonVal::Str(id));
        }
    }

    fn on_blur(
        &mut self,
        _state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
