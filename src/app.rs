use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::ComponentId,
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{
    add_header_window::{AddHeaderWindow, AddHeaderWindowState, ADD_HEADER_WINDOW_TEMPLATE},
    app_layout::AppLayoutComponent,
    app_section::{AppSection, AppSectionState, APP_SECTION_TEMPLATE},
    confirm_action_window::ConfirmActionWindow,
    dashboard::DashboardComponent,
    edit_header_selector::{
        EditHeaderSelector, EditHeaderSelectorState, EDIT_HEADER_SELECTOR_TEMPLATE,
    },
    edit_header_window::EditHeaderWindow,
    edit_input::EditInput,
    edit_name_textinput::EditNameTextInput,
    edit_value_textinput::EditValueTextInput,
    floating_windows::{edit_endpoint_name::EditEndpointName, edit_project_name::EditProjectName},
    focusable_section::{FocusableSection, FocusableSectionState},
    header_name_textinput::HeaderNameTextInput,
    header_value_textinput::HeaderValueTextInput,
    menu_item::{MenuItem, MenuItemState, MENU_ITEM_TEMPLATE},
    method_selector::{MethodSelector, MethodSelectorState, METHOD_SELECTOR_TEMPLATE},
    project_window::ProjectWindow,
    request_body_section::REQUEST_BODY_SECTION_TEMPLATE,
    request_headers_editor::{
        RequestHeadersEditor, RequestHeadersEditorState, REQUEST_HEADERS_EDITOR_TEMPLATE,
    },
    row::{Row, RowState, ROW_TEMPLATE},
    textarea::{TextArea, TextAreaInputState, TEXTAREA_TEMPLATE},
    textinput::{InputState, TextInput, TEXTINPUT_TEMPLATE},
};

pub fn app() -> anyhow::Result<()> {
    App::new().run()?;

    Ok(())
}

struct App {
    component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl App {
    pub fn new() -> Self {
        App {
            component_ids: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
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
        self.register_components(&mut runtime_builder)?;

        let runtime = runtime_builder.finish();
        if let Ok(mut runtime) = runtime {
            let _emitter = runtime.emitter();

            runtime.run();
        } else if let Err(error) = runtime {
            println!("{:?}", error);
        }

        Ok(())
    }

    // impl<T, G> RuntimeBuilder<T, G>
    // pub fn register_prototype<FC, FS, C>(&mut self, ident: impl Into<String>, template: impl ToSourceKind, proto: FC, state: FS) -> Result<()>
    // where
    //     FC: 'static + Fn() -> C,
    //     FS: 'static + FnMut() -> C::State,
    //     C: Component + 'static,
    //     // Bounds from impl:
    //     G: GlobalEvents,

    // fn get_prototypes<FC, FS, S>(
    //     &self,
    // ) -> Vec<(
    //     impl Into<String>,
    //     impl ToSourceKind,
    //     Box<dyn Fn() -> Box<dyn Component<State = S, Message = String>>>,
    //     Box<dyn FnMut() -> Box<dyn Component<State = S, Message = String>>>,
    // )>
    // where
    //     S: State,
    // {
    //     vec![
    //         (
    //             "textinput",
    //             TEXTINPUT_TEMPLATE,
    //             Box::new(|| Box::new(TextInput {})),
    //             Box::new(|| Box::new(InputState::new())),
    //         ),
    //         // (
    //         //     "textarea",
    //         //     TEXTAREA_TEMPLATE,
    //         //     Box::new(|| TextArea),
    //         //     Box::new(TextAreaInputState::new),
    //         // ),
    //     ]
    // }

    fn register_prototypes(
        &self,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let mut component_ids = self.component_ids.clone();

        builder.register_prototype(
            "url_input",
            "./src/components/templates/url_input.aml",
            || FocusableSection,
            FocusableSectionState::new,
        )?;

        builder.register_prototype(
            "textinput",
            TEXTINPUT_TEMPLATE,
            move || TextInput {
                component_ids: component_ids.clone(),
                listeners: vec!["dashboard".to_string()],
            },
            InputState::new,
        )?;

        component_ids = self.component_ids.clone();
        builder.register_prototype(
            "request_body_input",
            TEXTAREA_TEMPLATE,
            move || TextArea {
                component_ids: component_ids.clone(),
                listeners: vec!["dashboard".to_string()],
            },
            TextAreaInputState::new,
        )?;

        component_ids = self.component_ids.clone();
        builder.register_prototype(
            "textarea",
            TEXTAREA_TEMPLATE,
            move || TextArea {
                component_ids: component_ids.clone(),
                listeners: vec![],
            },
            TextAreaInputState::new,
        )?;

        builder.register_prototype(
            "method_selector",
            METHOD_SELECTOR_TEMPLATE,
            || MethodSelector,
            MethodSelectorState::new,
        )?;

        builder.register_prototype(
            "menu_item",
            MENU_ITEM_TEMPLATE,
            || MenuItem,
            MenuItemState::new,
        )?;

        builder.register_prototype(
            "request_headers_editor",
            REQUEST_HEADERS_EDITOR_TEMPLATE,
            || RequestHeadersEditor,
            RequestHeadersEditorState::new,
        )?;

        builder.register_prototype(
            "app_section",
            APP_SECTION_TEMPLATE,
            || AppSection,
            AppSectionState::new,
        )?;

        builder.register_prototype(
            "request_body_section",
            REQUEST_BODY_SECTION_TEMPLATE,
            || FocusableSection,
            FocusableSectionState::new,
        )?;

        builder.register_prototype(
            "add_header_window",
            ADD_HEADER_WINDOW_TEMPLATE,
            || AddHeaderWindow,
            AddHeaderWindowState::new,
        )?;

        builder.register_prototype(
            "edit_header_selector",
            EDIT_HEADER_SELECTOR_TEMPLATE,
            || EditHeaderSelector,
            EditHeaderSelectorState::new,
        )?;

        builder.register_prototype("row", ROW_TEMPLATE, || Row, RowState::new)?;

        Ok(())
    }

    fn register_components(
        &mut self,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        self.register_prototypes(builder)?;

        AppLayoutComponent::register(&self.component_ids, builder)?;
        EditHeaderWindow::register(&self.component_ids, builder)?;
        HeaderNameTextInput::register(&self.component_ids, builder)?;
        HeaderValueTextInput::register(&self.component_ids, builder)?;
        EditNameTextInput::register(&self.component_ids, builder)?;
        EditValueTextInput::register(&self.component_ids, builder)?;
        ProjectWindow::register(&self.component_ids, builder)?;
        ConfirmActionWindow::register(&self.component_ids, builder)?;
        DashboardComponent::register(&self.component_ids, builder)?;
        EditEndpointName::register(&self.component_ids, builder)?;
        EditInput::register(&self.component_ids, builder, "edit_endpoint_name_input")?;
        EditInput::register(&self.component_ids, builder, "edit_project_name_input")?;
        EditProjectName::register(&self.component_ids, builder)?;

        Ok(())
    }
}
