use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::ComponentId,
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::{CommonVal, State, Value},
    widgets::Elements,
};
use serde::{Deserialize, Serialize};

pub const APP_LAYOUT_TEMPLATE: &str = "./src/components/templates/app_layout.aml";

#[derive(Debug, Deserialize, Serialize)]
pub enum AppLayoutMessages {
    OpenOptions,
}

enum AppDisplay {
    Dashboard,
    Options,
}

impl State for AppDisplay {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        match self {
            AppDisplay::Dashboard => Some(CommonVal::Str("Dashboard")),
            AppDisplay::Options => Some(CommonVal::Str("Options")),
        }
    }
}

#[derive(anathema::state::State)]
pub struct AppLayoutState {
    display: Value<AppDisplay>,
}

pub struct AppLayoutComponent {
    #[allow(dead_code)]
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl AppLayoutComponent {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let app_id = builder.register_component(
            "app",
            APP_LAYOUT_TEMPLATE,
            AppLayoutComponent {
                component_ids: ids.clone(),
            },
            AppLayoutState {
                display: AppDisplay::Dashboard.into(),
            },
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("app"), app_id);

            new_map
        });

        Ok(())
    }
}

impl anathema::component::Component for AppLayoutComponent {
    type State = AppLayoutState;
    type Message = String;

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        context.set_focus("id", "app");
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _: Elements<'_, '_>,
        _: Context<'_, Self::State>,
    ) {
        let Ok(app_layout_message) = serde_json::from_str::<AppLayoutMessages>(&message) else {
            return;
        };

        match app_layout_message {
            AppLayoutMessages::OpenOptions => {
                state.display.set(AppDisplay::Options);
            }
        }
    }
}
