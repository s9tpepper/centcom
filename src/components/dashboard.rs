use anathema::{
    component::{ComponentId, KeyCode, KeyEvent},
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::{CommonVal, List, State, Value},
    widgets::Elements,
};
use std::fs;
use std::ops::Deref;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use arboard::Clipboard;
use serde::{Deserialize, Serialize};

use crate::fs::get_documents_dir;
use crate::projects::{
    save_project, Endpoint, HeaderState, PersistedEndpoint, PersistedProject, Project,
    DEFAULT_ENDPOINT_NAME, DEFAULT_PROJECT_NAME,
};
use crate::requests::do_request;

use super::{
    add_header_window::AddHeaderWindow,
    edit_header_selector::EditHeaderSelector,
    edit_header_window::EditHeaderWindow,
    floating_windows::{
        edit_endpoint_name::{EditEndpointName, EditEndpointNameMessages},
        edit_project_name::{EditProjectName, EditProjectNameMessages},
    },
    method_selector::MethodSelector,
    project_window::ProjectWindow,
    response_renderer::TextFilter,
    send_message,
    textarea::TextAreaMessages,
    textinput::TextInputMessages,
};
use super::{
    floating_windows::endpoints_selector::{EndpointsSelector, EndpointsSelectorMessages},
    response_renderer::ResponseRendererMessages,
};

pub const DASHBOARD_TEMPLATE: &str = "./src/components/templates/dashboard.aml";

#[derive(Copy, Clone)]
pub enum MainDisplay {
    RequestBody,
    RequestHeadersEditor,
    ResponseBody,
    ResponseHeaders,
}

impl anathema::state::State for MainDisplay {
    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            MainDisplay::RequestBody => Some(CommonVal::Str("request_body")),
            MainDisplay::RequestHeadersEditor => Some(CommonVal::Str("request_headers_editor")),
            MainDisplay::ResponseBody => Some(CommonVal::Str("response_body")),
            MainDisplay::ResponseHeaders => Some(CommonVal::Str("response_headers")),
        }
    }
}

#[derive(anathema::state::State)]
pub struct MenuItem {
    label: Value<String>,
}

#[derive(PartialEq, Eq)]
pub enum FloatingWindow {
    None,
    Method,
    AddHeader,
    EditHeader,
    Error,
    EditHeaderSelector,
    Project,
    ConfirmProject,
    Message,
    ChangeEndpointName,
    ChangeProjectName,
    EndpointsSelector,
}

impl State for FloatingWindow {
    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            FloatingWindow::None => Some(CommonVal::Str("None")),
            FloatingWindow::Method => Some(CommonVal::Str("Method")),
            FloatingWindow::AddHeader => Some(CommonVal::Str("AddHeader")),
            FloatingWindow::EditHeader => Some(CommonVal::Str("EditHeader")),
            FloatingWindow::Error => Some(CommonVal::Str("Error")),
            FloatingWindow::EditHeaderSelector => Some(CommonVal::Str("EditHeaderSelector")),
            FloatingWindow::Project => Some(CommonVal::Str("Project")),
            FloatingWindow::ConfirmProject => Some(CommonVal::Str("ConfirmProject")),
            FloatingWindow::Message => Some(CommonVal::Str("Message")),
            FloatingWindow::ChangeEndpointName => Some(CommonVal::Str("ChangeEndpointName")),
            FloatingWindow::ChangeProjectName => Some(CommonVal::Str("ChangeProjectName")),
            FloatingWindow::EndpointsSelector => Some(CommonVal::Str("EndpointsSelector")),
        }
    }
}

#[derive(anathema::state::State)]
pub struct DashboardState {
    pub main_display: Value<MainDisplay>,
    pub floating_window: Value<FloatingWindow>,

    pub endpoint: Value<Endpoint>,
    pub response_headers: Value<List<HeaderState>>,
    pub response: Value<String>,
    pub response_body_window_label: Value<String>,

    pub error_message: Value<String>,
    pub message: Value<String>,
    pub message_label: Value<String>,
    pub menu_items: Value<List<MenuItem>>,
    pub top_menu_items: Value<List<MenuItem>>,
    pub logs: Value<String>,

    pub new_header_name: Value<String>,
    pub new_header_value: Value<String>,

    pub edit_header_name: Value<String>,
    pub edit_header_value: Value<String>,

    pub header_being_edited: Value<Option<Value<HeaderState>>>,

    pub project: Value<Project>,
    // pub project_count: Value<u8>,
    pub endpoint_count: Value<u8>,

    pub filter_indexes: Value<List<usize>>,
    pub filter_total: Value<usize>,
    pub filter_nav_index: Value<usize>,
}

impl DashboardState {
    pub fn new() -> Self {
        let project = Project::new();

        DashboardState {
            // project_count: 0.into(),
            project: project.into(),
            endpoint_count: 0.into(),
            endpoint: Endpoint::new().into(),

            response: "".to_string().into(),
            message: "".to_string().into(),
            message_label: "".to_string().into(),
            response_body_window_label: "".to_string().into(),
            error_message: "".to_string().into(),
            new_header_name: "".to_string().into(),
            new_header_value: "".to_string().into(),
            edit_header_name: "".to_string().into(),
            edit_header_value: "".to_string().into(),
            floating_window: FloatingWindow::None.into(),
            main_display: Value::<MainDisplay>::new(MainDisplay::RequestBody),
            logs: "".to_string().into(),
            menu_items: List::from_iter([
                MenuItem {
                    label: "(S)ave Project".to_string().into(),
                },
                MenuItem {
                    label: "Save Endpo(i)nt".to_string().into(),
                },
                MenuItem {
                    label: "New Project".to_string().into(),
                },
                MenuItem {
                    label: "New Endpoint".to_string().into(),
                },
                MenuItem {
                    label: "(O)ptions".to_string().into(),
                },
            ]),
            top_menu_items: List::from_iter([MenuItem {
                label: "(P)rojects".to_string().into(),
            }]),
            response_headers: List::from_iter(vec![]),
            header_being_edited: None.into(),
            filter_indexes: List::empty(),
            filter_total: 0.into(),
            filter_nav_index: 0.into(),
        }
    }
}

pub struct DashboardComponent {
    pub component_ids: Rc<RefCell<HashMap<String, ComponentId<String>>>>,
}

impl DashboardComponent {
    pub fn register(
        ids: &Rc<RefCell<HashMap<String, ComponentId<String>>>>,
        builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        let id = builder.register_component(
            "dashboard",
            DASHBOARD_TEMPLATE,
            DashboardComponent {
                component_ids: ids.clone(),
            },
            DashboardState::new(),
        )?;

        let ids_ref = ids.clone();
        ids_ref.replace_with(|old| {
            let mut new_map = old.clone();
            new_map.insert(String::from("dashboard"), id);

            new_map
        });

        Ok(())
    }

    fn show_message(&self, title: &str, message: &str, state: &mut DashboardState) {
        state.message.set(message.to_string());
        state.message_label.set(title.to_string());
        state.floating_window.set(FloatingWindow::Message);

        // TODO: Add same auto-close behavior as show_error()
    }

    fn show_error(&self, message: &str, state: &mut DashboardState) {
        state.error_message.set(message.to_string());
        state.floating_window.set(FloatingWindow::Error);

        // NOTE: Can not sleep thread, as it prevents anathema from displaying
        // the window change. This needs to be in it's own thread if I want to
        // close the window after a certain amount of time
        //
        // let close_delay = Duration::from_secs(5);
        // sleep(close_delay);
        // state.floating_window.set(FloatingWindow::None);
    }

    fn save_project(&self, state: &mut DashboardState) {
        let project: PersistedProject = state.project.to_ref().deref().into();

        match save_project(project) {
            Ok(_) => self.show_message("Project Save", "Saved project successfully", state),
            Err(error) => self.show_error(&error.to_string(), state),
        }
    }

    fn save_endpoint(&self, state: &mut DashboardState, _: Context<'_, DashboardState>) {
        let project_name = state.project.to_ref().name.to_ref().to_string();
        let endpoint_name = state.endpoint.to_ref().name.to_ref().to_string();

        if endpoint_name == DEFAULT_ENDPOINT_NAME {
            self.show_error("Please give your endpoint a name to save", state);
            return;
        }

        if project_name == DEFAULT_PROJECT_NAME {
            self.show_error("Please give your project a name to save", state);
            return;
        }

        let mut project: PersistedProject = state.project.to_ref().deref().into();
        let existing_endpoint = project
            .endpoints
            .iter()
            .enumerate()
            .find(|(_, endpoint)| endpoint.name == endpoint_name);

        if let Some((index, _)) = existing_endpoint {
            project.endpoints.remove(index);
        }

        project
            .endpoints
            .push((&state.endpoint.to_ref().clone()).into());

        match save_project(project.clone()) {
            Ok(_) => {
                self.show_message("Endoint Save", "Saved endpoint successfully", state);
                state.project.set((&project).into());
            }
            Err(error) => self.show_error(&error.to_string(), state),
        }
    }

    fn save_response_body(&self, state: &mut DashboardState, _: Context<'_, DashboardState>) {
        let dir = get_documents_dir();

        match dir {
            Ok(mut docs_dir) => {
                let response = state.response.to_ref().to_string();

                let endpoint_name = state.endpoint.to_ref().name.to_ref().to_string();
                let endpoint_name = endpoint_name.replace("/", "_");

                let timestamp = SystemTime::now();
                let duration = timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(1));
                let name = format!("{endpoint_name}_{}.txt", duration.as_secs());

                docs_dir.push(name);
                let save_path = docs_dir.clone();

                match fs::write(docs_dir, response) {
                    Ok(_) => {
                        self.show_message(
                            "Response Saved",
                            format!("Saved to {save_path:?}").as_str(),
                            state,
                        );
                    }
                    Err(err) => self.show_error(&err.to_string(), state),
                }
            }
            Err(error) => self.show_error(&error.to_string(), state),
        }
    }

    fn open_edit_project_name_window(
        &self,
        state: &mut DashboardState,
        mut context: Context<'_, DashboardState>,
    ) {
        state.floating_window.set(FloatingWindow::ChangeProjectName);

        context.set_focus("id", "edit_project_name");

        if let Ok(ids) = self.component_ids.try_borrow() {
            let mut input_value = state.project.to_ref().name.to_ref().clone();
            if input_value == DEFAULT_PROJECT_NAME {
                input_value = String::new();
            }

            let message = EditProjectNameMessages::InputValue(input_value);
            let _ = serde_json::to_string(&message).map(|msg| {
                let _ = send_message("edit_project_name", msg, ids, context.emitter.clone());
            });
        }
    }

    fn open_endpoints_selector(
        &self,
        state: &mut DashboardState,
        mut context: Context<'_, DashboardState>,
    ) {
        state.floating_window.set(FloatingWindow::EndpointsSelector);
        context.set_focus("id", "endpoints_selector");

        let persisted_endpoints: Vec<PersistedEndpoint> = state
            .project
            .to_ref()
            .endpoints
            .to_ref()
            .iter()
            .map(|endpoint| {
                let e = endpoint.to_ref();

                (&*e).into()
            })
            .collect();

        let msg = EndpointsSelectorMessages::EndpointsList(persisted_endpoints);
        #[allow(clippy::single_match)]
        match self.component_ids.try_borrow() {
            #[allow(clippy::single_match)]
            Ok(ids) => match ids.get("endpoints_selector") {
                Some(id) => {
                    let _ = serde_json::to_string(&msg).map(|payload| context.emit(*id, payload));
                }
                None => self.show_error("Unable to find endpoints window id", state),
            },
            Err(_) => self.show_error("Unable to find components id map", state),
        };
    }

    fn open_edit_endpoint_name_window(
        &self,
        state: &mut DashboardState,
        mut context: Context<'_, DashboardState>,
    ) {
        state
            .floating_window
            .set(FloatingWindow::ChangeEndpointName);
        context.set_focus("id", "edit_endpoint_name");

        if let Ok(ids) = self.component_ids.try_borrow() {
            let mut input_value = state.endpoint.to_ref().name.to_ref().clone();
            if input_value == DEFAULT_ENDPOINT_NAME {
                input_value = String::new();
            }

            let message = EditEndpointNameMessages::InputValue(input_value);
            let _ = serde_json::to_string(&message).map(|msg| {
                let _ = send_message("edit_endpoint_name", msg, ids, context.emitter.clone());
            });
        }
    }

    fn yank_response(&self, state: &mut DashboardState) {
        let Ok(mut clipboard) = Clipboard::new() else {
            self.show_error("Error accessing your clipboard", state);

            return;
        };

        let operation_text = state.response.to_ref().clone();
        let set_operation = clipboard.set();
        match set_operation.text(operation_text) {
            Ok(_) => self.show_message("Clipboard", "Response copied to clipboard", state),
            Err(error) => self.show_error(&error.to_string(), state),
        }
    }

    fn get_text_filter(&self, state: &mut DashboardState) -> TextFilter {
        let mut indexes: Vec<usize> = vec![];

        let i = state.filter_indexes.to_ref();
        i.iter().for_each(|e| {
            let val = *e.to_ref();
            indexes.push(val);
        });

        TextFilter {
            indexes,
            total: *state.filter_total.to_ref(),
            nav_index: *state.filter_nav_index.to_ref(),
        }
    }

    fn sync_text_filter(&self, state: &mut DashboardState, context: Context<'_, DashboardState>) {
        let text_filter = self.get_text_filter(state);

        if let Ok(message) =
            serde_json::to_string(&ResponseRendererMessages::FilterUpdate(text_filter))
        {
            let emitter = context.emitter.clone();
            if let Ok(component_ids) = self.component_ids.try_borrow() {
                let _ = send_message("response_renderer", message, component_ids, emitter);
            }
        };
    }

    fn apply_response_filter(
        &self,
        value: CommonVal<'_>,
        state: &mut DashboardState,
        context: Context<'_, DashboardState>,
    ) {
        loop {
            if state.filter_indexes.len() == 0 {
                break;
            }

            state.filter_indexes.remove(0);
        }
        state.filter_total.set(0);
        state.filter_nav_index.set(0);

        let filter = value.to_string();
        if filter.is_empty() {
            self.sync_text_filter(state, context);

            return;
        }

        let response = state.response.to_ref().to_string();
        response.lines().enumerate().for_each(|(idx, line)| {
            if line.contains(&filter) {
                state.filter_indexes.push(idx);
            }
        });
        state.filter_total.set(state.filter_indexes.len());

        if state.filter_indexes.len() > 0 {
            self.sync_text_filter(state, context);
        }
    }
}

pub trait DashboardMessageHandler {
    fn handle_message(
        value: CommonVal<'_>,
        ident: impl Into<String>,
        state: &mut DashboardState,
        context: Context<'_, DashboardState>,
        component_ids: Ref<'_, HashMap<String, ComponentId<String>>>,
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DashboardMessages {
    TextInput(TextInputMessages),
    TextArea(TextAreaMessages),
}

impl anathema::component::Component for DashboardComponent {
    type State = DashboardState;
    type Message = String;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _: Elements<'_, '_>,
        _: Context<'_, Self::State>,
    ) {
        if let Ok(dashboard_message) = serde_json::from_str::<DashboardMessages>(&message) {
            match dashboard_message {
                DashboardMessages::TextInput(text_input_message) => match text_input_message {
                    TextInputMessages::InputChange(value) => {
                        let mut endpoint = state.endpoint.to_mut();
                        let name_still_default = *endpoint.url.to_ref() == *endpoint.name.to_ref();

                        endpoint.url.set(value.to_string());

                        if name_still_default {
                            endpoint.name.set(value.to_string());
                        }
                    }
                },

                DashboardMessages::TextArea(text_area_message) => match text_area_message {
                    TextAreaMessages::InputChange(value) => {
                        state.endpoint.to_mut().body.set(value);
                    }
                },
            }
        }
    }

    fn receive(
        &mut self,
        ident: &str,
        value: CommonVal<'_>,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        #[allow(clippy::single_match)]
        match ident {
            "url_input_focus" => {
                context.set_focus("id", "app");
            }

            _ => {}
        }

        let (component, event) = ident.split_once("__").unwrap_or(("", ""));
        if let Ok(component_ids) = self.component_ids.try_borrow() {
            match component {
                "response_filter" => match event {
                    "input_update" => self.apply_response_filter(value, state, context),

                    "input_escape" => context.set_focus("id", "response_renderer"),

                    _ => {}
                },

                "add_header" => {
                    AddHeaderWindow::handle_message(value, ident, state, context, component_ids);
                }

                "edit_header" => {
                    EditHeaderWindow::handle_message(value, ident, state, context, component_ids);
                }

                "edit_header_selector" => {
                    EditHeaderSelector::handle_message(value, ident, state, context, component_ids);
                }

                "method_selector" => {
                    MethodSelector::handle_message(value, ident, state, context, component_ids);
                }

                "project_window" => {
                    ProjectWindow::handle_message(value, ident, state, context, component_ids);
                }

                "edit_endpoint_name" => {
                    EditEndpointName::handle_message(value, ident, state, context, component_ids);
                }

                "edit_project_name" => {
                    EditProjectName::handle_message(value, ident, state, context, component_ids);
                }

                "endpoints_selector" => {
                    EndpointsSelector::handle_message(value, ident, state, context, component_ids);
                }

                _ => {}
            }
        } else {
            println!("Could not find id for {ident}");
        }
    }

    fn on_key(
        &mut self,
        event: KeyEvent,
        state: &mut Self::State,
        elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match event.code {
            KeyCode::Char(char) => {
                let main_display = *state.main_display.to_ref();

                match char {
                    's' => self.save_project(state),
                    'n' => self.open_edit_endpoint_name_window(state, context),
                    'j' => self.open_edit_project_name_window(state, context),
                    'i' => self.save_endpoint(state, context),
                    'f' => context.set_focus("id", "response_body_input"),

                    'v' => match main_display {
                        MainDisplay::RequestBody => {}
                        MainDisplay::RequestHeadersEditor => {}
                        MainDisplay::ResponseBody => self.save_response_body(state, context),
                        MainDisplay::ResponseHeaders => {}
                    },

                    // Set focus to the request url text input
                    'u' => {
                        if !event.ctrl {
                            context.set_focus("id", "url_input");
                        }
                    }

                    // Quit app
                    'q' => quit::with_code(0),

                    // Make the request
                    'r' => do_request(state, context, elements, self),

                    // Show request body editor window
                    'b' => match main_display {
                        MainDisplay::RequestBody => context.set_focus("id", "textarea"),
                        MainDisplay::RequestHeadersEditor => {
                            state.main_display.set(MainDisplay::RequestBody);
                        }
                        MainDisplay::ResponseBody => {
                            state.main_display.set(MainDisplay::RequestBody);
                        }
                        MainDisplay::ResponseHeaders => {
                            state.main_display.set(MainDisplay::ResponseBody)
                        }
                    },

                    // Show request headers editor window
                    'd' => {
                        if !event.ctrl {
                            state.main_display.set(MainDisplay::RequestHeadersEditor);
                        }
                    }

                    // Open Endpoints selector
                    'e' => self.open_endpoints_selector(state, context),

                    // Show projects window
                    'p' => {
                        state.floating_window.set(FloatingWindow::Project);
                        if let Ok(component_ids) = self.component_ids.try_borrow() {
                            if let Some(id) = component_ids.get("project_window") {
                                context.emit(*id, "load".to_string());
                            }
                        }

                        context.set_focus("id", "project_window");
                    }

                    // Show response headers display
                    'h' => match main_display {
                        MainDisplay::RequestBody => {}
                        MainDisplay::RequestHeadersEditor => {
                            state
                                .floating_window
                                .set(FloatingWindow::EditHeaderSelector);
                            context.set_focus("id", "edit_header_selector");
                        }
                        MainDisplay::ResponseBody => {
                            state.main_display.set(MainDisplay::ResponseHeaders)
                        }
                        MainDisplay::ResponseHeaders => {}
                    },

                    // Open Request Method selection window
                    'm' => {
                        state.floating_window.set(FloatingWindow::Method);
                        context.set_focus("id", "method_selector");
                    }

                    'a' => match main_display {
                        MainDisplay::RequestBody => {}
                        MainDisplay::RequestHeadersEditor => {
                            // Open header window
                            state.floating_window.set(FloatingWindow::AddHeader);
                            context.set_focus("id", "add_header_window");
                        }
                        MainDisplay::ResponseBody => {}
                        MainDisplay::ResponseHeaders => {}
                    },

                    'y' => match main_display {
                        MainDisplay::RequestBody => {}
                        MainDisplay::RequestHeadersEditor => {}
                        MainDisplay::ResponseBody => {
                            // Copy response body to clipboard
                            self.yank_response(state)
                        }
                        MainDisplay::ResponseHeaders => {}
                    },

                    _ => {}
                }
            }

            KeyCode::Esc => {
                context.set_focus("id", "app");

                if *state.floating_window.to_ref() != FloatingWindow::None {
                    state.floating_window.set(FloatingWindow::None);
                }
            }

            KeyCode::Enter => {
                // TODO: Do something with the Enter button
            }

            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}
