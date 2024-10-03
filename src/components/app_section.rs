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

    fn receive(
        &mut self,
        ident: &str,
        value: CommonVal<'_>,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        println!("app_section received message ident: {ident}");
        if ident == "input_focus" {
            let focus = value.to_bool();
            println!("app_section received message focus: {focus}");

            let section_id = state.section_id.to_ref().clone();
            let Some(section_id) = section_id else { return };
            let section_id = section_id.to_string().leak();

            match focus {
                true => elements
                    .by_attribute("id", CommonVal::Str(section_id))
                    .each(|_element, attributes| {
                        attributes.set("foreground", "#ffffff");
                    }),
                false => elements
                    .by_attribute("id", CommonVal::Str(section_id))
                    .each(|_element, attributes| {
                        attributes.set("foreground", "#ff0000");
                    }),
            }
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}
