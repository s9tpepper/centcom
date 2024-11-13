use std::fs;
use std::ops::Deref;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use anathema::{
    component::{ComponentId, KeyCode, KeyEvent},
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::{CommonVal, List, State, Value},
    widgets::Elements,
};

use arboard::Clipboard;
use serde::{Deserialize, Serialize};

use crate::fs::get_documents_dir;
use crate::projects::{save_project, Endpoint, HeaderState, PersistedProject, Project};

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
    send_message,
    textarea::TextAreaMessages,
    textinput::TextInputMessages,
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
    pub current_project: Value<String>,
    pub project_count: Value<u8>,
    pub endpoint_count: Value<u8>,

    pub selected_project: Value<Option<Project>>,
}

impl DashboardState {
    pub fn new() -> Self {
        DashboardState {
            project_count: 0.into(),
            endpoint_count: 0.into(),
            current_project: "[None]".to_string().into(),

            project: Project::new().into(),
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
                    label: "(S)ave".to_string().into(),
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
            selected_project: None.into(),
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
            let input_value = state.project.to_ref().name.to_ref().clone();
            let message = EditProjectNameMessages::InputValue(input_value);
            let _ = serde_json::to_string(&message).map(|msg| {
                let _ = send_message("edit_project_name", msg, ids, context.emitter.clone());
            });
        }
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
            let input_value = state.endpoint.to_ref().name.to_ref().clone();
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
        context: Context<'_, Self::State>,
    ) {
        let (component, _) = ident.split_once("__").unwrap_or(("", ""));
        if let Ok(component_ids) = self.component_ids.try_borrow() {
            match component {
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

                    'v' => match main_display {
                        MainDisplay::RequestBody => {}
                        MainDisplay::RequestHeadersEditor => {}
                        MainDisplay::ResponseBody => self.save_response_body(state, context),
                        MainDisplay::ResponseHeaders => {}
                    },

                    // Set focus to the request url text input
                    'u' => context.set_focus("id", "url_input"),

                    // Quit app
                    'q' => quit::with_code(0),

                    // Make the request
                    'r' => do_request(state, context, elements),

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

                    'e' => {
                        state
                            .floating_window
                            .set(FloatingWindow::EditHeaderSelector);
                        context.set_focus("id", "edit_header_selector");
                    }

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
                    'h' => state.main_display.set(MainDisplay::ResponseHeaders),

                    // Open Request Method selection window
                    'm' => {
                        state.floating_window.set(FloatingWindow::Method);
                        context.set_focus("id", "method_selector");
                    }

                    // Open header window
                    'a' => {
                        state.floating_window.set(FloatingWindow::AddHeader);
                        context.set_focus("id", "add_header_window");
                    }

                    // Copy response body to clipboard
                    'y' => self.yank_response(state),

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

// TODO: Move this to a module so its not in the dashboard component
fn do_request(
    state: &mut DashboardState,
    mut context: anathema::prelude::Context<'_, DashboardState>,
    _elements: anathema::widgets::Elements<'_, '_>,
) {
    let endpoint = state.endpoint.to_ref();
    let url = endpoint.url.to_ref().clone();
    let method = endpoint.method.to_ref().clone();
    let headers = endpoint.headers.to_ref();

    let mut content_type = String::new();
    let mut request_builder = http::Request::builder();
    for header_value in headers.iter() {
        let header = header_value.to_ref();
        let header_name = header.name.to_ref().to_string();
        let header_value = header.value.to_ref().to_string();

        if header_name.to_lowercase() == "content-type" {
            content_type.push_str(header_value.as_str());
        }

        request_builder = request_builder.header(header_name, header_value);
    }

    let http_request_result = request_builder
        .method(method.as_str())
        .uri(url.as_str())
        .body(vec![0u8]);

    if http_request_result.is_err() {
        let error = http_request_result.unwrap_err();
        state.error_message.set(error.to_string());
        state.floating_window.set(FloatingWindow::Error);
        return;
    }

    let http_request = http_request_result.unwrap();
    let (http_parts, _body) = http_request.into_parts();
    let request: ureq::Request = http_parts.into();
    // let response = request.send_bytes(&body);
    // let request_body = state.in
    let response = match content_type.as_str() {
        "application/json" => {
            let req_body = endpoint.body.to_ref().clone();

            request.send_string(&req_body)
        }

        // TODO: Figure out how to support form k/v pairs in the request body builder interface
        // "multipart/form" => request.send_form("")
        //
        _ => request.send_string(""),
    };

    if let Ok(response) = response {
        let status = response.status();

        loop {
            if state.response_headers.len() > 0 {
                state.response_headers.pop_back();
            } else {
                break;
            }
        }

        for name in response.headers_names() {
            let Some(value) = response.header(&name) else {
                continue;
            };

            state.response_headers.push(HeaderState {
                name: name.into(),
                value: value.to_string().into(),
            });
        }

        let body = response
            .into_string()
            .unwrap_or("Could not read response body".to_string());

        let window_label = format!("Response Body (Status Code: {status})");
        state.response.set(body);
        state.response_body_window_label.set(window_label);
        state.main_display.set(MainDisplay::ResponseBody);

        context.set_focus("id", "response");
    } else {
        let error = response.unwrap_err();

        state.error_message.set(error.to_string());
        state.floating_window.set(FloatingWindow::Error);
    }
}
