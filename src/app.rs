use std::collections::HashMap;

use anathema::{
    component::ComponentId,
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{
    add_header_window::{AddHeaderWindow, AddHeaderWindowState, ADD_HEADER_WINDOW_TEMPLATE},
    app_layout::{AppLayoutComponent, AppLayoutState, APP_LAYOUT_TEMPLATE},
    app_section::{AppSection, AppSectionState, APP_SECTION_TEMPLATE},
    dashboard::{DashboardComponent, DashboardState, DASHBOARD_TEMPLATE},
    edit_header_selector::{
        EditHeaderSelector, EditHeaderSelectorState, EDIT_HEADER_SELECTOR_TEMPLATE,
    },
    edit_header_window::{EditHeaderWindow, EditHeaderWindowState, EDIT_HEADER_WINDOW_TEMPLATE},
    edit_name_textinput::{EditNameInputState, EditNameTextInput, EDIT_NAME_INPUT_TEMPLATE},
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

struct App {
    component_ids: HashMap<String, ComponentId<String>>,
}

impl App {
    pub fn new() -> Self {
        App {
            component_ids: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let doc = Document::new("@app");

        let tui = TuiBackend::builder()
            // .enable_alt_screen()
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

        let runtime = runtime_builder.finish();
        if let Ok(mut runtime) = runtime {
            let _emitter = runtime.emitter();

            runtime.run();
        } else if let Err(error) = runtime {
            println!("{:?}", error);
        }

        Ok(())
    }

    fn register_components(&mut self, builder: &mut RuntimeBuilder<TuiBackend, ()>) {
        let _ = builder.register_prototype(
            "url_input",
            "./src/components/templates/url_input.aml",
            || FocusableSection,
            FocusableSectionState::new,
        );

        let _ = builder.register_prototype(
            "textinput",
            TEXTINPUT_TEMPLATE,
            || TextInput {
                input_override: None,
            },
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

        let _ = builder.register_prototype(
            "add_header_window",
            ADD_HEADER_WINDOW_TEMPLATE,
            || AddHeaderWindow,
            AddHeaderWindowState::new,
        );

        let edit_header_window_id = builder.register_component(
            "edit_header_window",
            EDIT_HEADER_WINDOW_TEMPLATE,
            EditHeaderWindow,
            EditHeaderWindowState::new(),
        );

        if let Ok(edit_header_window_id) = edit_header_window_id {
            self.component_ids
                .insert("edit_header_window".to_string(), edit_header_window_id);
        }

        let _ = builder.register_prototype(
            "edit_header_selector",
            EDIT_HEADER_SELECTOR_TEMPLATE,
            || EditHeaderSelector,
            EditHeaderSelectorState::new,
        );

        let _ = builder.register_prototype("row", ROW_TEMPLATE, || Row, RowState::new);

        let edit_header_name_id = builder.register_component(
            "editheadername",
            EDIT_NAME_INPUT_TEMPLATE,
            EditNameTextInput,
            EditNameInputState::new(),
        );
        if let Ok(edit_header_name_id) = edit_header_name_id {
            self.component_ids
                .insert("edit_header_name_input".to_string(), edit_header_name_id);
        }

        let dashboard = DashboardComponent {
            component_ids: self.component_ids.clone(),
        };

        let dashboard_id = builder.register_component(
            "dashboard",
            DASHBOARD_TEMPLATE,
            dashboard,
            DashboardState::new(),
        );

        if let Ok(dashboard_id) = dashboard_id {
            self.component_ids
                .insert("dashboard".to_string(), dashboard_id);
        }
    }
}
