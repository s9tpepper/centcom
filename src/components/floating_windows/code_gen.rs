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
    code_gen::generate_typescript,
    components::{
        dashboard::{DashboardMessageHandler, DashboardMessages, DashboardState, FloatingWindow},
        send_message,
    },
    theme::{get_app_theme, AppTheme},
};

const TEMPLATE: &str = "./src/components/floating_windows/templates/code_gen.aml";

#[derive(Default)]
pub struct CodeGen {
    languages: HashMap<char, String>,
}

impl CodeGen {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let mut languages = HashMap::<char, String>::new();
        languages.insert('r', String::from("rust"));
        languages.insert('t', String::from("typescript"));
        languages.insert('j', String::from("javascript"));

        let app_id = builder.register_component(
            "codegen_window",
            TEMPLATE,
            CodeGen { languages },
            CodeGenState::new(),
        )?;

        let mut ids_ref = ids.borrow_mut();
        ids_ref.insert("codegen_window".to_string(), app_id);

        Ok(())
    }

    #[allow(unused)]
    fn update_app_theme(&self, state: &mut CodeGenState) {
        let app_theme = get_app_theme();
        state.app_theme.set(app_theme);
    }
}

#[derive(Default, State)]
pub struct CodeGenState {
    app_theme: Value<AppTheme>,
    language: Value<String>,
}

impl CodeGenState {
    pub fn new() -> Self {
        let app_theme = get_app_theme();

        CodeGenState {
            app_theme: app_theme.into(),
            language: "".to_string().into(),
        }
    }
}

impl Component for CodeGen {
    type State = CodeGenState;
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
                let default_language = "rust".to_string();
                let language = self.languages.get(&char).unwrap_or(&default_language);
                state.language.set(language.clone());

                context.publish("codegen__selection", |state| &state.language);
            }

            anathema::component::KeyCode::Esc => {
                context.publish("codegen__cancel", |state| &state.app_theme);
            }

            _ => {}
        }
    }
}

impl DashboardMessageHandler for CodeGen {
    fn handle_message(
        value: component::CommonVal<'_>,
        ident: impl Into<String>,
        state: &mut DashboardState,
        mut context: anathema::prelude::Context<'_, DashboardState>,
        _: Elements<'_, '_>,
        component_ids: Ref<'_, HashMap<String, ComponentId<String>>>,
    ) {
        let event: String = ident.into();
        match event.as_str() {
            #[allow(clippy::single_match)]
            "codegen__selection" => match value.to_string().as_str() {
                "rust" => {
                    state.floating_window.set(FloatingWindow::None);

                    // state.project

                    println!("Generate rust code here");

                    // TODO: Set this to the code generation dialog instead of app
                    context.set_focus("id", "app");
                }

                "typescript" => {
                    state.floating_window.set(FloatingWindow::None);
                    context.set_focus("id", "app");

                    let project = state.project.to_ref();

                    match generate_typescript((&*project).into()) {
                        Ok(_) => {
                            let title = String::from("TypeScript Code Gen");
                            let msg = String::from("Code generated successfully");
                            let message = DashboardMessages::ShowSucces((title, msg));

                            let _ = serde_json::to_string(&message).map(|message| {
                                let _ = send_message(
                                    "dashboard",
                                    message,
                                    &component_ids,
                                    context.emitter,
                                );
                            });
                        }

                        Err(_) => {
                            let msg = String::from("Error generating TypeScript code");
                            let message = DashboardMessages::ShowError(msg);

                            let _ = serde_json::to_string(&message).map(|message| {
                                let _ = send_message(
                                    "dashboard",
                                    message,
                                    &component_ids,
                                    context.emitter,
                                );
                            });
                        }
                    }
                }

                "javascript" => {
                    state.floating_window.set(FloatingWindow::None);

                    println!("Generate javascript code here");

                    // TODO: Set this to the code generation dialog instead of app
                    context.set_focus("id", "app");
                }

                _ => {}
            },

            "codegen__cancel" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");
            }

            _ => {}
        }
    }
}
