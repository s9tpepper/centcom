use crate::options::Options;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{State, Value},
};

use crate::options::get_options;

use super::app_layout::AppLayoutMessages;

const TEMPLATE: &str = "./src/components/templates/options.aml";

#[derive(Default)]
pub struct OptionsView {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

#[derive(Default, State)]
pub struct OptionsViewState {
    options: Value<OptionsState>,
}

#[derive(Default, State)]
struct OptionsState {
    syntax_theme: Value<String>,
}

impl From<Options> for OptionsState {
    fn from(val: Options) -> Self {
        OptionsState {
            syntax_theme: val.syntax_theme.into(),
        }
    }
}

impl OptionsViewState {
    pub fn new(options: Options) -> Self {
        let options_state: OptionsState = options.into();
        OptionsViewState {
            options: options_state.into(),
        }
    }
}

impl OptionsView {
    pub fn new(component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>) -> Self {
        OptionsView { component_ids }
    }

    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let options = get_options();
        let id = builder.register_component(
            "options",
            TEMPLATE,
            OptionsView::new(ids.clone()),
            OptionsViewState::new(options),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("options"), id);

            new_map
        });

        Ok(())
    }

    fn go_back(&self, context: anathema::prelude::Context<'_, OptionsViewState>) {
        let component_ids = self.component_ids.try_borrow();
        if component_ids.is_err() {
            return;
        }

        let component_ids = component_ids.unwrap();
        let Some(app_id) = component_ids.get("app") else {
            return;
        };

        let Ok(msg) = serde_json::to_string(&AppLayoutMessages::OpenDashboard) else {
            return;
        };

        context.emit(*app_id, msg);
    }
}

impl Component for OptionsView {
    type State = OptionsViewState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match key.code {
            #[allow(clippy::single_match)]
            anathema::component::KeyCode::Char(char) => match char {
                'b' => self.go_back(context),
                'x' => println!("open theme selector"),

                _ => {}
            },
            anathema::component::KeyCode::Esc => self.go_back(context),

            _ => {}
        }
    }
}
