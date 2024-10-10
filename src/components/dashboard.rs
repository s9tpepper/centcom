use anathema::{
    component::{KeyCode, KeyEvent},
    prelude::Context,
    state::{CommonVal, List, State, Value},
    widgets::Elements,
};

use arboard::Clipboard;

use crate::components::request_headers_editor::Header;

pub const DASHBOARD_TEMPLATE: &str = "./src/components/templates/dashboard.aml";

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

#[derive(anathema::state::State)]
pub struct DashboardState {
    url: Value<String>,
    method: Value<String>,
    request_headers: Value<List<Header>>,
    request_body: Value<String>,
    response_headers: Value<List<Header>>,
    response: Value<String>,
    response_body_window_label: Value<String>,
    show_method_window: Value<bool>,
    show_add_header_window: Value<bool>,
    show_error_window: Value<bool>,
    error_message: Value<String>,
    show_message_window: Value<bool>,
    message: Value<String>,
    message_label: Value<String>,
    main_display: Value<MainDisplay>,
    menu_items: Value<List<MenuItem>>,
    logs: Value<String>,
    new_header_name: Value<String>,
    new_header_value: Value<String>,
}

impl DashboardState {
    pub fn new() -> Self {
        DashboardState {
            url: "".to_string().into(),
            method: "GET".to_string().into(),
            response: "".to_string().into(),
            message: "".to_string().into(),
            message_label: "".to_string().into(),
            response_body_window_label: "".to_string().into(),
            error_message: "".to_string().into(),
            new_header_name: "".to_string().into(),
            new_header_value: "".to_string().into(),
            show_method_window: false.into(),
            show_add_header_window: false.into(),
            show_error_window: false.into(),
            show_message_window: false.into(),
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
            request_headers: List::from_iter(get_default_headers()),
            request_body: "".to_string().into(),
            response_headers: List::from_iter(vec![]),
        }
    }
}

pub struct DashboardComponent;
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

                let header = Header {
                    name: header_name.into(),
                    value: header_value.into(),
                };

                state.request_headers.push(header);
                state.show_add_header_window.set(false);

                context.set_focus("id", "app");
            }

            "cancel_add_header" => {
                state.show_add_header_window.set(false);
                state.new_header_name.set("".to_string());
                state.new_header_value.set("".to_string());
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
                state.show_method_window.set(false);
            }

            "new_method_selection" => {
                let value = &*value.to_common_str();

                state.method.set(value.to_string());

                // Trigger a resize on the text input by setting focus and then resetting it to app
                context.set_focus("id", "url_input");
                context.set_focus("id", "app");
            }

            "header_name_update" => {
                let new_header_name = value.to_string();

                state.new_header_name.set(new_header_name);
            }
            "header_value_update" => state.new_header_value.set(value.to_string()),

            "request_body_update" => state.request_body.set(value.to_string()),

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
                match char {
                    'n' => context.set_focus("id", "header_name_input"),
                    'v' => context.set_focus("id", "header_value_input"),

                    'u' => context.set_focus("id", "url_input"),
                    'q' => context.set_focus("id", "textarea"),

                    'r' => do_request(state, context, elements),
                    'b' => state.main_display.set(MainDisplay::RequestBody),
                    'd' => state.main_display.set(MainDisplay::RequestHeadersEditor),
                    'p' => {
                        state.main_display.set(MainDisplay::ResponseBody);
                        context.set_focus("id", "response");
                    }
                    'h' => state.main_display.set(MainDisplay::ResponseHeaders),

                    // floating windows
                    'm' => {
                        state.show_method_window.set(true);
                        context.set_focus("id", "method_selector");
                    }
                    'a' => {
                        state.show_add_header_window.set(true);
                        context.set_focus("id", "add_header_window");
                    }
                    'y' => {
                        let Ok(mut clipboard) = Clipboard::new() else {
                            state
                                .error_message
                                .set("Error accessing your clipboard".to_string());
                            state.show_error_window.set(true);
                            return;
                        };

                        let set_operation = clipboard.set();
                        match set_operation.text(state.response.to_ref().clone()) {
                            Ok(_) => {
                                state
                                    .message
                                    .set("Response copied to clipboard".to_string());
                                state.message_label.set("Clipboard".to_string());
                                state.show_message_window.set(true);
                            }

                            Err(error) => {
                                state.error_message.set(error.to_string());
                                state.show_error_window.set(true);
                            }
                        }
                    }
                    _ => {}
                }
            }

            KeyCode::Esc => {
                context.set_focus("id", "app");

                if state.show_error_window.to_ref().to_bool() {
                    state.show_error_window.set(false);
                }

                if state.show_message_window.to_ref().to_bool() {
                    state.show_message_window.set(false);
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
        state.show_error_window.set(true);
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

            state.response_headers.push(Header {
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
        state.show_error_window.set(true);
    }

    context.set_focus("id", "app");
}

fn get_default_headers() -> Vec<Header> {
    vec![
        Header {
            name: "user-agent".to_string().into(),
            value: "centcom-tui".to_string().into(),
        },
        Header {
            name: "content-type".to_string().into(),
            value: "application/json".to_string().into(),
        },
    ]
}
