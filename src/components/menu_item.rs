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

    fn accept_focus(&self) -> bool {
        false
    }
}
