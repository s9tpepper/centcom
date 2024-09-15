use anathema::{
    prelude::{Context, Document, TuiBackend},
    runtime::Runtime,
    state::{CommonVal, Value},
    widgets::Elements,
};

use crate::components::textinput::{InputState, TextInput, TEXTINPUT_TEMPLATE};

const DASHBOARD_TEMPLATE: &str = "./src/dashboard/templates/dashboard.aml";

pub fn dashboard() {
    let _ = Dashboard::new().run();
}

#[derive(anathema::state::State)]
struct DashboardState {
    input: Value<String>,
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
        if ident == "input_update" {
            let _value = &*value.to_common_str();

            // NOTE: value is updated input from textinput
            // println!("Input update: {value}");
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        context.set_focus("id", 1);

        // Get mutable access to the name
        let mut input = state.input.to_mut();

        match key.code {
            anathema::component::KeyCode::Char(char) => {
                input.push(char);
            }

            anathema::component::KeyCode::Backspace => {
                input.pop();
            }

            anathema::component::KeyCode::Delete => {
                input.remove(0);
            }

            _ => {}
        }
    }
}

struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {}
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let template = read_to_string(DASHBOARD_TEMPLATE)?;
        let doc = Document::new("@dashboard");

        // let dashboard_id = doc.add_component("dashboard", SourceKind::from(DASHBOARD_TEMPLATE));

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

        let _ = runtime_builder.register_component(
            "textinput",
            TEXTINPUT_TEMPLATE,
            TextInput,
            InputState::new(),
        );

        let _dashboard_id = runtime_builder.register_component(
            "dashboard",
            DASHBOARD_TEMPLATE,
            DashboardComponent,
            DashboardState {
                input: "".to_string().into(),
            },
        );

        let mut runtime = runtime_builder.finish().unwrap();

        let _emitter = runtime.emitter();

        runtime.run();

        Ok(())
    }
}
