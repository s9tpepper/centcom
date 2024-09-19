use anathema::{prelude::Context, widgets::Elements};

pub const APP_LAYOUT_TEMPLATE: &str = "./src/components/templates/app_layout.aml";

#[derive(anathema::state::State)]
pub struct AppLayoutState {}

pub struct AppLayoutComponent;
impl anathema::component::Component for AppLayoutComponent {
    type State = AppLayoutState;
    type Message = ();

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        context.set_focus("id", "app");
    }
}
