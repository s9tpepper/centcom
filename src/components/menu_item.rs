use anathema::{
    component::Component,
    state::{State, Value},
};

pub const MENU_ITEM_TEMPLATE: &str = "./src/components/templates/menu_item.aml";

#[derive(Default)]
pub struct MenuItem;

#[derive(Default, State)]
pub struct MenuItemState {
    label: Value<String>,
    key_binding: Value<char>,
}

impl MenuItemState {
    pub fn new() -> Self {
        MenuItemState {
            label: "".to_string().into(),
            key_binding: ' '.into(),
        }
    }
}

impl Component for MenuItem {
    type State = MenuItemState;
    type Message = ();

    // fn on_key(
    //     &mut self,
    //     key: anathema::component::KeyEvent,
    //     state: &mut Self::State,
    //     mut elements: anathema::widgets::Elements<'_, '_>,
    //     mut context: anathema::prelude::Context<'_, Self::State>,
    // ) {
    //     let item = context.get_external("item");
    // }
}
