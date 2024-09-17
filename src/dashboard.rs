use anathema::{
    component::{KeyCode, KeyEvent},
    prelude::{Context, Document, TuiBackend},
    runtime::Runtime,
    state::{CommonVal, Value},
    widgets::Elements,
};

use crate::components::textinput::{InputState, TextInput, TEXTINPUT_TEMPLATE};

const APP_TEMPLATE: &str = "./src/app.aml";
const DASHBOARD_TEMPLATE: &str = "./src/dashboard/templates/dashboard.aml";

pub fn dashboard() {
    let _ = Dashboard::new().run();
}

#[derive(anathema::state::State)]
struct AppState {}

struct AppComponent;
impl anathema::component::Component for AppComponent {
    type State = AppState;
    type Message = ();

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        context.set_focus("id", "app");
    }
}

#[derive(anathema::state::State)]
struct DashboardState {
    show_method_window: Value<bool>,
}

impl DashboardState {
    pub fn new() -> Self {
        DashboardState {
            show_method_window: false.into(),
        }
    }
}

#[derive(Debug)]
struct DashboardUpdate {
    // data: String,
}

struct DashboardComponent;
impl anathema::component::Component for DashboardComponent {
    type State = DashboardState;
    type Message = DashboardUpdate;

    fn receive(
        &mut self,
        ident: &str,
        value: CommonVal<'_>,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        if ident == "url_update" {
            // TODO: Do something with url updates (put it in some kind of state)
            let _value = &*value.to_common_str();

            // NOTE: value is updated input from textinput
            // println!("Input update: {value}");
        }
    }

    fn on_key(
        &mut self,
        event: KeyEvent,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match event.code {
            KeyCode::Char(char) => {
                if char == 'u' {
                    context.set_focus("id", 1);
                }

                if char == 'm' {
                    state.show_method_window.set(true);
                }
            }

            KeyCode::Enter => todo!(),
            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

struct Dashboard {}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {}
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let doc = Document::new("@app");

        let tui = TuiBackend::builder()
            .enable_alt_screen()
            .enable_raw_mode()
            .hide_cursor()
            .finish();

        if let Err(ref error) = tui {
            println!("[ERROR] Could not start terminal interface");
            println!("{error:?}");
        }

        let backend = tui.unwrap();
        let mut runtime_builder = Runtime::builder(doc, backend);

        let _ = runtime_builder.register_prototype(
            "textinput",
            TEXTINPUT_TEMPLATE,
            || TextInput,
            InputState::new,
        );

        let _dashboard_id = runtime_builder.register_prototype(
            "dashboard",
            DASHBOARD_TEMPLATE,
            || DashboardComponent,
            DashboardState::new,
        );

        let _ = runtime_builder.register_component("app", APP_TEMPLATE, AppComponent, AppState {});

        let mut runtime = runtime_builder.finish().unwrap();

        let _emitter = runtime.emitter();

        runtime.run();

        Ok(())
    }
}
