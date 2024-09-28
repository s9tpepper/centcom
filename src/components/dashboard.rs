use anathema::{
    component::{ComponentId, KeyCode, KeyEvent},
    prelude::{Context, WidgetComponentId},
    state::{CommonVal, List, Value},
    widgets::Elements,
};

use super::textarea::TextArea;

pub const DASHBOARD_TEMPLATE: &str = "./src/components/templates/dashboard.aml";

#[derive(anathema::state::State)]
struct MenuItem {
    label: Value<String>,
}

#[derive(anathema::state::State)]
pub struct DashboardState {
    url: Value<String>,
    method: Value<String>,
    response: Value<String>,
    show_method_window: Value<bool>,
    menu_items: Value<List<MenuItem>>,
    logs: Value<String>,
}

impl DashboardState {
    pub fn new() -> Self {
        DashboardState {
            url: "".to_string().into(),
            method: "GET".to_string().into(),
            response: "".to_string().into(),
            show_method_window: false.into(),
            logs: "".to_string().into(),
            menu_items: List::from_iter([
                MenuItem {
                    label: "(U)RL".to_string().into(),
                },
                MenuItem {
                    label: "(M)ethod".to_string().into(),
                },
                MenuItem {
                    label: "Add (H)eader".to_string().into(),
                },
                MenuItem {
                    label: "Send (R)equest".to_string().into(),
                },
                MenuItem {
                    label: "(S)ave".to_string().into(),
                },
                MenuItem {
                    label: "(O)ptions".to_string().into(),
                },
            ]),
        }
    }
}

#[derive(Debug)]
pub struct DashboardUpdate {
    // data: String,
}

pub struct DashboardComponent;
impl anathema::component::Component for DashboardComponent {
    type State = DashboardState;
    type Message = DashboardUpdate;

    fn receive(
        &mut self,
        ident: &str,
        value: CommonVal<'_>,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        match ident {
            "log_output" => {
                let value = &*value.to_common_str();
                let mut logs = state.logs.to_mut();
                logs.insert_str(0, value);
            }

            "url_update" => {
                // TODO: Do something with url updates (put it in some kind of state)
                let value = &*value.to_common_str();
                state.url.set(value.to_string());
            }

            "cancel_method_selector" => {
                state.show_method_window.set(false);
            }

            "new_method_selection" => {
                let value = &*value.to_common_str();

                state.method.set(value.to_string());
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
            KeyCode::Char(char) => match char {
                'u' => context.set_focus("id", 1),

                'm' => {
                    state.show_method_window.set(true);
                    context.set_focus("id", "method_selector")
                }

                'r' => do_request(state, context, elements),

                _ => {}
            },

            KeyCode::Enter => todo!(),
            _ => {}
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

fn do_request(
    state: &mut DashboardState,
    mut _context: anathema::prelude::Context<'_, DashboardState>,
    _elements: anathema::widgets::Elements<'_, '_>,
) {
    let url = state.url.to_ref().clone();
    let method = state.method.to_ref().clone();

    let http_request = http::Request::builder()
        .method(method.as_str())
        .uri(url.as_str())
        .body(vec![0u8])
        .unwrap();
    let (http_parts, _body) = http_request.into_parts();
    let request: ureq::Request = http_parts.into();
    // let response = request.send_bytes(&body);

    let response = request.send_string("");
    if let Ok(response) = response {
        let _status = response.status();
        let body = response
            .into_string()
            .unwrap_or("Could not read response body".to_string());

        state.response.set(body);
    }
}
