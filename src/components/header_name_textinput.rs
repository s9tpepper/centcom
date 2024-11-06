use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::ComponentId,
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    widgets::Elements,
};

use super::textinput::{InputReceiver, InputState, TEXTINPUT_TEMPLATE};

#[derive(Default)]
pub struct HeaderNameTextInput {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl HeaderNameTextInput {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let id = builder.register_component(
            "headernameinput",
            TEXTINPUT_TEMPLATE,
            HeaderNameTextInput {
                component_ids: ids.clone(),
            },
            InputState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("headernameinput"), id);

            new_map
        });

        Ok(())
    }
}

impl anathema::component::Component for HeaderNameTextInput {
    type State = InputState;
    type Message = String;

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        context.set_focus("id", "add_header_window");

        self._on_blur(state, elements, context);
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        self._on_focus(state, elements, context);
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        self._on_key(key, state, elements, context);
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        elements: Elements<'_, '_>,
        context: Context<'_, Self::State>,
    ) {
        self._message(message, state, elements, context);
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

impl InputReceiver for HeaderNameTextInput {}
