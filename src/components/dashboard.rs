use anathema::{
    component::{KeyCode, KeyEvent},
    prelude::Context,
    state::{CommonVal, List, Value},
    widgets::Elements,
};

use crate::components::request_headers_editor::Header;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use syntect::{easy::HighlightLines, parsing::SyntaxReference};

pub const DASHBOARD_TEMPLATE: &str = "./src/components/templates/dashboard.aml";

enum MainDisplay {
    RequestBody,
    RequestHeadersEditor,
    ResponseBody,
}

impl anathema::state::State for MainDisplay {
    fn to_common(&self) -> Option<CommonVal<'_>> {
        match self {
            MainDisplay::RequestBody => Some(CommonVal::Str("request_body")),
            MainDisplay::RequestHeadersEditor => Some(CommonVal::Str("request_headers_editor")),
            MainDisplay::ResponseBody => Some(CommonVal::Str("response_body")),
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
    response: Value<String>,
    show_method_window: Value<bool>,
    show_add_header_window: Value<bool>,
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
            new_header_name: "".to_string().into(),
            new_header_value: "".to_string().into(),
            show_method_window: false.into(),
            show_add_header_window: false.into(),
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
        }
    }
}

pub struct DashboardComponent {
    syntax_set: Option<SyntaxSet>,
    theme_set: Option<ThemeSet>,
}

impl DashboardComponent {
    pub fn new() -> DashboardComponent {
        DashboardComponent {
            syntax_set: Some(SyntaxSet::load_defaults_newlines()),
            theme_set: Some(ThemeSet::load_defaults()),
        }
    }

    fn do_request(
        &self,
        state: &mut DashboardState,
        mut context: anathema::prelude::Context<'_, DashboardState>,
        _: anathema::widgets::Elements<'_, '_>,
    ) {
        let url = state.url.to_ref().clone();
        let method = state.method.to_ref().clone();
        let headers = state.request_headers.to_ref();

        let mut request_builder = http::Request::builder();
        for header_value in headers.iter() {
            let header = header_value.to_ref();
            let header_name = header.name.to_ref().to_string();
            let header_value = header.value.to_ref().to_string();

            request_builder = request_builder.header(header_name, header_value);
        }

        let http_request_result = request_builder
            .method(method.as_str())
            .uri(url.as_str())
            .body(vec![0u8]);

        if let Err(http_request_error) = http_request_result {
            dbg!(http_request_error);
            println!("url: {}", url);
            return;
        }

        let http_request = http_request_result.unwrap();
        let (http_parts, _body) = http_request.into_parts();
        let request: ureq::Request = http_parts.into();
        // let response = request.send_bytes(&body);

        let response = request.call();
        if response.is_err() {
            return;
        }

        let response = response.unwrap();
        // let _status = response.status();

        // let content_type: Option<&str> = Some("text/plain");
        // let mut c: &str = response.content_type();
        let content_type: &str = "text/html";

        let mut body = String::new();
        let mut reader = response.into_reader();
        let _ = reader.read_to_string(&mut body);

        // let mut body = response
        //     .into_string()
        //     .unwrap_or("Could not read response body".to_string());

        // let content_type = &mut response.header("content-type");
        // response.

        println!("Checking");
        if let Some((_, content_type)) = content_type.split_once("/") {
            println!("Adding syntax highlighting...");

            if self.syntax_set.is_none() || self.theme_set.is_none() {
                return;
            }

            let syntax_set = self.syntax_set.as_ref().unwrap();

            let syntax: Option<&SyntaxReference> = if content_type != "plain" {
                syntax_set.find_syntax_by_extension(content_type)
            } else {
                Some(syntax_set.find_syntax_plain_text())
            };

            if syntax.is_none() {
                return;
            }

            let theme_set = self.theme_set.as_ref().unwrap();
            let syntax = syntax.unwrap();
            let mut highlighter =
                HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

            let raw_body = body.clone();
            body.clear();

            for line in LinesWithEndings::from(&raw_body) {
                let ranges: Vec<(Style, &str)> =
                    highlighter.highlight_line(line, syntax_set).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                body.push_str(&escaped);
            }
        }

        state.response.set(body.to_string());
        state.main_display.set(MainDisplay::ResponseBody);

        context.set_focus("id", "app");
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
        // Load these once at the start of your program
        // let ps = SyntaxSet::load_defaults_newlines();
        // let ts = ThemeSet::load_defaults();

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
                println!("header_name_update: {new_header_name}");

                state.new_header_name.set(new_header_name);
            }
            "header_value_update" => state.new_header_value.set(value.to_string()),

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
            KeyCode::Char(char) => match char {
                'n' => context.set_focus("id", "header_name_input"),
                'v' => context.set_focus("id", "header_value_input"),

                'u' => context.set_focus("id", "url_input"),
                'q' => context.set_focus("id", "textarea"),

                'r' => self.do_request(state, context, elements),
                'b' => state.main_display.set(MainDisplay::RequestBody),
                'd' => state.main_display.set(MainDisplay::RequestHeadersEditor),

                // floating windows
                'm' => {
                    state.show_method_window.set(true);
                    context.set_focus("id", "method_selector");
                }
                'a' => {
                    state.show_add_header_window.set(true);
                    context.set_focus("id", "add_header_window");
                }
                _ => {}
            },

            KeyCode::Esc => context.set_focus("id", "app"),

            KeyCode::Enter => todo!(),
            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
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
