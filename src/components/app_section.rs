use anathema::{
    component::Component,
    state::{State, Value},
};

pub const APP_SECTION_TEMPLATE: &str = "./src/components/templates/app_section.aml";

#[derive(Default)]
pub struct AppSection;

#[derive(Default, State)]
pub struct AppSectionState {
    // border_color: Value<String>,
    // top_label: Value<Option<String>>,
    // bottom_label: Value<Option<String>>,
}

impl AppSectionState {
    pub fn new() -> Self {
        AppSectionState {
            // border_color: "#ffffff".to_string().into(),
            // top_label: None.into(),
            // bottom_label: None.into(),
        }
    }
}

impl Component for AppSection {
    type State = AppSectionState;
    type Message = ();
}
