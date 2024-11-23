use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::State,
};

const TEMPLATE: &str = "./src/components/templates/options.aml";

#[derive(Default)]
pub struct Options {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl Options {
    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        Options { component_ids }
    }

    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let id = builder.register_component(
            "options",
            TEMPLATE,
            Options::new(ids.clone()),
            OptionsState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("options"), id);

            new_map
        });

        Ok(())
    }
}

#[derive(Default, State)]
pub struct OptionsState {}

impl OptionsState {
    pub fn new() -> Self {
        OptionsState {}
    }
}

impl Component for Options {
    type State = OptionsState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        false
    }
}
