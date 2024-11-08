use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::CommonVal,
    widgets::Elements,
};

const TEMPLATE: &str = "./src/components/templates/edit_input.aml";

use super::inputs::{InputReceiver, InputState};

#[derive(Default)]
pub struct EditInput {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl EditInput {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
        ident: impl Into<String>,
    ) -> anyhow::Result<()> {
        let name: String = ident.into();
        let app_id = builder.register_component(
            name.clone(),
            TEMPLATE,
            EditInput {
                component_ids: ids.clone(),
            },
            InputState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(name, app_id);

            new_map
        });

        Ok(())
    }
}

impl InputReceiver for EditInput {}
impl Component for EditInput {
    type State = InputState;
    type Message = String;

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
