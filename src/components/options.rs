use crate::options::Options;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{Component, ComponentId},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{CommonVal, State, Value},
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
    options_window: Value<OptionsWindows>,
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
            options_window: OptionsWindows::None.into(),
        }
    }
}

#[derive(Default)]
enum OptionsWindows {
    #[default]
    None,
    SyntaxThemeSelector,
}

impl State for OptionsWindows {
    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            OptionsWindows::SyntaxThemeSelector => Some(CommonVal::Str("SyntaxThemeSelector")),
            OptionsWindows::None => Some(CommonVal::Str("None")),
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

    fn open_theme_selector(
        &self,
        state: &mut OptionsViewState,
        mut context: anathema::prelude::Context<'_, OptionsViewState>,
    ) {
        state
            .options_window
            .set(OptionsWindows::SyntaxThemeSelector);

        context.set_focus("id", "syntax_theme_selector");
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
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match key.code {
            #[allow(clippy::single_match)]
            anathema::component::KeyCode::Char(char) => match char {
                'b' => self.go_back(context),
                'x' => self.open_theme_selector(state, context),

                _ => {}
            },
            anathema::component::KeyCode::Esc => self.go_back(context),

            _ => {}
        }
    }

    fn receive(
        &mut self,
        ident: &str,
        _value: CommonVal<'_>,
        state: &mut Self::State,
        _: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match ident {
            "syntax_theme_selector__selection" => {
                // TODO: Grab new selection and save it to the options
            }
            "syntax_theme_selector__cancel" => {
                state.options_window.set(OptionsWindows::None);
                context.set_focus("id", "options");
            }

            _ => {}
        }
    }
}
