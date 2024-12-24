use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use anathema::{
    component::{self, Component, ComponentId},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{State, Value},
    widgets::Elements,
};

use crate::{
    components::dashboard::{DashboardMessageHandler, DashboardState, FloatingWindow},
    theme::{get_app_theme, AppTheme},
};

const TEMPLATE: &str = "./src/components/floating_windows/templates/commands.aml";

#[derive(Default)]
pub struct Commands;

impl Commands {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let app_id = builder.register_component(
            "commands_window",
            TEMPLATE,
            Commands {},
            CommandsState::new(),
        )?;

        let mut ids_ref = ids.borrow_mut();
        ids_ref.insert("commands_window".to_string(), app_id);

        Ok(())
    }

    #[allow(unused)]
    fn update_app_theme(&self, state: &mut CommandsState) {
        let app_theme = get_app_theme();
        state.app_theme.set(app_theme);
    }
}

#[derive(Default, State)]
pub struct CommandsState {
    app_theme: Value<AppTheme>,
    command: Value<char>,
}

impl CommandsState {
    pub fn new() -> Self {
        let app_theme = get_app_theme();

        CommandsState {
            app_theme: app_theme.into(),
            command: ' '.into(),
        }
    }
}

impl Component for Commands {
    type State = CommandsState;
    type Message = String;

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        _: anathema::prelude::Context<'_, Self::State>,
    ) {
        self.update_app_theme(state);
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(char) => {
                state.command.set(char);
                context.publish("commands__selection", |state| &state.command);
            }

            anathema::component::KeyCode::Esc => {
                context.publish("commands__cancel", |state| &state.command);
            }

            _ => {}
        }
    }
}

impl DashboardMessageHandler for Commands {
    fn handle_message(
        value: component::CommonVal<'_>,
        ident: impl Into<String>,
        state: &mut DashboardState,
        mut context: anathema::prelude::Context<'_, DashboardState>,
        _: Elements<'_, '_>,
        _component_ids: Ref<'_, HashMap<String, ComponentId<String>>>,
    ) {
        let event: String = ident.into();

        match event.as_str() {
            #[allow(clippy::single_match)]
            "commands__selection" => match value.to_string().as_str() {
                "g" => {
                    state.floating_window.set(FloatingWindow::CodeGen);
                    context.set_focus("id", "codegen_window");
                }

                _ => {}
            },

            "commands__cancel" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");
            }

            _ => {}
        }
    }
}
