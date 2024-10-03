use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{
    app_layout::{AppLayoutComponent, AppLayoutState, APP_LAYOUT_TEMPLATE},
    app_section::{AppSection, AppSectionState, APP_SECTION_TEMPLATE},
    dashboard::{DashboardComponent, DashboardState, DASHBOARD_TEMPLATE},
    focusable_section::{FocusableSection, FocusableSectionState},
    menu_item::{MenuItem, MenuItemState, MENU_ITEM_TEMPLATE},
    method_selector::{MethodSelector, MethodSelectorState, METHOD_SELECTOR_TEMPLATE},
    request_body_section::REQUEST_BODY_SECTION_TEMPLATE,
    request_headers_editor::{
        RequestHeadersEditor, RequestHeadersEditorState, REQUEST_HEADERS_EDITOR_TEMPLATE,
    },
    row::{Row, RowState, ROW_TEMPLATE},
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
        let _dashboard_id = builder.register_component(
            "dashboard",
            DASHBOARD_TEMPLATE,
            DashboardComponent,
            DashboardState::new(),
        );

        let _ = builder.register_prototype(
            "url_input",
            "./src/components/templates/url_input.aml",
            || FocusableSection,
            FocusableSectionState::new,
        );

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

        let _ = builder.register_prototype(
            "request_headers_editor",
            REQUEST_HEADERS_EDITOR_TEMPLATE,
            || RequestHeadersEditor,
            RequestHeadersEditorState::new,
        );

        let _ = builder.register_prototype(
            "app_section",
            APP_SECTION_TEMPLATE,
            || AppSection,
            AppSectionState::new,
        );

        let _ = builder.register_prototype(
            "request_body_section",
            REQUEST_BODY_SECTION_TEMPLATE,
            || FocusableSection,
            FocusableSectionState::new,
        );

        let _ = builder.register_prototype("row", ROW_TEMPLATE, || Row, RowState::new);
    }
}
