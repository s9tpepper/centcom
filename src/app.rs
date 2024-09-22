use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{
    app_layout::{AppLayoutComponent, AppLayoutState, APP_LAYOUT_TEMPLATE},
    dashboard::{DashboardComponent, DashboardState, DASHBOARD_TEMPLATE},
    menu_item::{MenuItem, MenuItemState, MENU_ITEM_TEMPLATE},
    method_selector::{MethodSelector, MethodSelectorState, METHOD_SELECTOR_TEMPLATE},
    textarea::{TextArea, TextAreaInputState, TEXTAREA_TEMPLATE},
    textinput::{InputState, TextInput, TEXTINPUT_TEMPLATE},
};

pub fn app() {
    let _ = App::new().run();
}

struct App {}

impl App {
    pub fn new() -> Self {
        App {}
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
        self.register_components(&mut runtime_builder);

        let mut runtime = runtime_builder.finish().unwrap();

        let _emitter = runtime.emitter();

        runtime.run();

        Ok(())
    }

    fn register_components(&self, builder: &mut RuntimeBuilder<TuiBackend, ()>) {
        let _ = builder.register_prototype(
            "textinput",
            TEXTINPUT_TEMPLATE,
            || TextInput,
            InputState::new,
        );

        let _ = builder.register_prototype(
            "textarea",
            TEXTAREA_TEMPLATE,
            || TextArea,
            TextAreaInputState::new,
        );

        let _ = builder.register_prototype(
            "method_selector",
            METHOD_SELECTOR_TEMPLATE,
            || MethodSelector,
            MethodSelectorState::new,
        );

        let _dashboard_id = builder.register_prototype(
            "dashboard",
            DASHBOARD_TEMPLATE,
            || DashboardComponent,
            DashboardState::new,
        );

        let _ = builder.register_component(
            "app",
            APP_LAYOUT_TEMPLATE,
            AppLayoutComponent,
            AppLayoutState {},
        );

        let _ = builder.register_prototype(
            "menu_item",
            MENU_ITEM_TEMPLATE,
            || MenuItem,
            MenuItemState::new,
        );
    }
}
