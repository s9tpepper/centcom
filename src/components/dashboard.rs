use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anathema::{
    component::{ComponentId, KeyCode, KeyEvent},
    prelude::{Context, TuiBackend},
    runtime::RuntimeBuilder,
    state::{CommonVal, List, State, Value},
    widgets::Elements,
};

use arboard::Clipboard;

use crate::{
    components::request_headers_editor::HeaderState,
    messages::confirm_delete_project::ConfirmDeleteProject,
};

use super::project_window::{Project, ProjectState};

pub const DASHBOARD_TEMPLATE: &str = "./src/components/templates/dashboard.aml";

#[derive(Copy, Clone)]
enum MainDisplay {
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
struct MenuItem {
    label: Value<String>,
}

#[derive(PartialEq, Eq)]
enum FloatingWindow {
    None,
    Method,
    AddHeader,
    EditHeader,
    Error,
    EditHeaderSelector,
    Project,
    ConfirmProject,
    Message,
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
        }
    }
}

#[derive(anathema::state::State)]
pub struct DashboardState {
    main_display: Value<MainDisplay>,
    floating_window: Value<FloatingWindow>,
    url: Value<String>,
    method: Value<String>,
    request_headers: Value<List<HeaderState>>,
    request_body: Value<String>,
    response_headers: Value<List<HeaderState>>,
    response: Value<String>,
    response_body_window_label: Value<String>,
    error_message: Value<String>,
    message: Value<String>,
    message_label: Value<String>,
    menu_items: Value<List<MenuItem>>,
    top_menu_items: Value<List<MenuItem>>,
    logs: Value<String>,
    new_header_name: Value<String>,
    new_header_value: Value<String>,
    edit_header_name: Value<String>,
    edit_header_value: Value<String>,
    current_project: Value<String>,
    header_being_edited: Value<Option<Value<HeaderState>>>,
    project_count: Value<u8>,
    selected_project: Value<Option<ProjectState>>,
}

impl DashboardState {
    pub fn new() -> Self {
        DashboardState {
            project_count: 0.into(),
            current_project: "[None]".to_string().into(),
            url: "".to_string().into(),
            method: "GET".to_string().into(),
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
            request_headers: List::from_iter(get_default_headers()),
            request_body: "".to_string().into(),
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
}

impl anathema::component::Component for DashboardComponent {
    type State = DashboardState;
    type Message = String;

    fn receive(
        &mut self,
        ident: &str,
        value: CommonVal<'_>,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        mut context: Context<'_, Self::State>,
    ) {
        match ident {
            "add_header" => {
                let header_name = state.new_header_name.to_ref().to_string();
                let header_value = state.new_header_value.to_ref().to_string();

                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");

                if header_name.trim().is_empty() || header_value.trim().is_empty() {
                    return;
                }

                let header = HeaderState {
                    name: header_name.into(),
                    value: header_value.into(),
                };
                state.request_headers.push(header);
            }

            "edit_header" => {
                let header_name = state.edit_header_name.to_ref().to_string();
                let header_value = state.edit_header_value.to_ref().to_string();

                let header = HeaderState {
                    name: header_name.into(),
                    value: header_value.into(),
                };

                state.request_headers.push(header);
                state.floating_window.set(FloatingWindow::None);

                context.set_focus("id", "app");
            }

            "cancel_add_header" => {
                state.floating_window.set(FloatingWindow::None);
                state.new_header_name.set("".to_string());
                state.new_header_value.set("".to_string());
                context.set_focus("id", "app");
            }

            "cancel_edit_header" => {
                state.floating_window.set(FloatingWindow::None);
                state.edit_header_name.set("".to_string());
                state.edit_header_value.set("".to_string());

                let header = state.header_being_edited.to_mut();
                let header = header.as_ref();
                if let Some(header) = header {
                    state.request_headers.push(HeaderState {
                        name: header.to_ref().name.to_ref().clone().into(),
                        value: header.to_ref().value.to_ref().clone().into(),
                    });
                }

                context.set_focus("id", "app");
            }

            "log_output" => {
                let value = &*value.to_common_str();
                let mut logs = state.logs.to_mut();
                logs.insert_str(0, value);
            }

            "url_update" => {
                let value = &*value.to_common_str();
                state.url.set(value.to_string());
            }

            "cancel_method_selector" => {
                state.floating_window.set(FloatingWindow::None);
            }

            "new_method_selection" => {
                let value = &*value.to_common_str();

                state.method.set(value.to_string());

                // Trigger a resize on the text input by setting focus and then resetting it to app
                context.set_focus("id", "url_input");
                context.set_focus("id", "app");
            }

            "header_name_update" => state.new_header_name.set(value.to_string()),
            "header_value_update" => state.new_header_value.set(value.to_string()),

            "edit_header_name_update" => state.edit_header_name.set(value.to_string()),
            "edit_header_value_update" => state.edit_header_value.set(value.to_string()),

            "request_body_update" => state.request_body.set(value.to_string()),

            "cancel_header_selection" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");
            }

            "header_selection" => {
                let selection: usize = value.to_string().parse().unwrap();
                let header = state.request_headers.remove(selection);
                if let Some(selected_header) = &header {
                    let header = selected_header.to_ref();
                    state.edit_header_name.set(header.name.to_ref().clone());
                    state.edit_header_value.set(header.value.to_ref().clone());
                };

                state.header_being_edited.set(header);

                state.floating_window.set(FloatingWindow::EditHeader);

                let component_ids = self.component_ids.try_borrow();
                if let Ok(component_ids) = component_ids {
                    let edit_header_name_input_id = component_ids.get("edit_header_name_input");
                    if let Some(id) = edit_header_name_input_id {
                        context.emit(*id, state.edit_header_name.to_ref().clone());
                    }

                    let edit_header_value_input_id = component_ids.get("edit_header_value_input");
                    if let Some(id) = edit_header_value_input_id {
                        context.emit(*id, state.edit_header_value.to_ref().clone());
                    }
                }

                context.set_focus("id", "edit_header_window");
            }

            "cancel_project_window" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");
            }

            "project_selection" => {
                state.floating_window.set(FloatingWindow::None);
                context.set_focus("id", "app");

                let value = &*value.to_common_str();
                let project = serde_json::from_str::<Project>(value);

                match project {
                    Ok(project) => {
                        state.current_project.set(project.name.clone());
                        state.selected_project.set(Some(project.into()));
                    }
                    Err(_) => todo!(),
                }
            }

            "delete_project" => {
                state.floating_window.set(FloatingWindow::ConfirmProject);

                let value = &*value.to_common_str();
                let project = serde_json::from_str::<Project>(value);

                match project {
                    Ok(project) => {
                        let confirm_message = ConfirmDeleteProject {
                            title: format!("Delete {}", project.name),
                            message: "Are you sure you want to delete?".into(),
                            project,
                        };

                        if let Ok(message) = serde_json::to_string(&confirm_message) {
                            if let Ok(component_ids) = self.component_ids.try_borrow() {
                                let confirm_action_window_id =
                                    component_ids.get("confirm_action_window");
                                if let Some(id) = confirm_action_window_id {
                                    context.emit(*id, message);
                                }
                            }
                        }
                    }
                    Err(_) => todo!(),
                }
            }

            _ => {}
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
                    // Set focus to the request url text input
                    'u' => context.set_focus("id", "url_input"),

                    // Set focus to the request body textarea
                    'q' => context.set_focus("id", "textarea"),

                    // Make the request
                    'r' => do_request(state, context, elements),

                    // Show request body editor window
                    'b' => match main_display {
                        MainDisplay::RequestBody => {}
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
                    'd' => state.main_display.set(MainDisplay::RequestHeadersEditor),

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
                    'y' => {
                        let Ok(mut clipboard) = Clipboard::new() else {
                            state
                                .error_message
                                .set("Error accessing your clipboard".to_string());
                            state.floating_window.set(FloatingWindow::Error);
                            return;
                        };

                        let set_operation = clipboard.set();
                        match set_operation.text(state.response.to_ref().clone()) {
                            Ok(_) => {
                                state
                                    .message
                                    .set("Response copied to clipboard".to_string());
                                state.message_label.set("Clipboard".to_string());
                                state.floating_window.set(FloatingWindow::Message);
                            }

                            Err(error) => {
                                state.error_message.set(error.to_string());
                                state.floating_window.set(FloatingWindow::Error);
                            }
                        }
                    }
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

fn do_request(
    state: &mut DashboardState,
    mut context: anathema::prelude::Context<'_, DashboardState>,
    _elements: anathema::widgets::Elements<'_, '_>,
) {
    let url = state.url.to_ref().clone();
    let method = state.method.to_ref().clone();
    let headers = state.request_headers.to_ref();

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
            let req_body = state.request_body.to_ref().clone();

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

    context.set_focus("id", "app");
}

fn get_default_headers() -> Vec<HeaderState> {
    vec![
        HeaderState {
            name: "user-agent".to_string().into(),
            value: "centcom-tui".to_string().into(),
        },
        HeaderState {
            name: "content-type".to_string().into(),
            value: "application/json".to_string().into(),
        },
    ]
}
