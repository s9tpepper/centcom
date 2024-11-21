use crate::{
    components::{
        dashboard::{DashboardComponent, DashboardState, FloatingWindow, MainDisplay},
        response_renderer::ResponseRendererMessages,
        send_message,
    },
    projects::HeaderState,
};

pub fn do_request(
    state: &mut DashboardState,
    mut context: anathema::prelude::Context<'_, DashboardState>,
    _: anathema::widgets::Elements<'_, '_>,
    dashboard: &mut DashboardComponent,
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

    match response {
        Ok(response) => {
            let status = response.status();

            loop {
                if state.response_headers.len() > 0 {
                    state.response_headers.pop_back();
                } else {
                    break;
                }
            }

            let mut ext = String::from("txt");
            for name in response.headers_names() {
                let Some(value) = response.header(&name) else {
                    continue;
                };

                if name.to_lowercase() == "content-type" {
                    if let Some((_, extension)) = value.to_string().split_once("/") {
                        ext = extension.to_string();
                    }
                }

                state.response_headers.push(HeaderState {
                    name: name.clone().into(),
                    value: value.to_string().clone().into(),
                });
            }

            let body = response.into_string();
            if body.is_err() {
                println!("Cant read body: {body:?}");
            }

            let body = body.unwrap_or("Could not read response body".to_string());

            let window_label = format!("Response Body (Status Code: {status})");

            state.response.set(body.to_string());
            state.response_body_window_label.set(window_label);
            state.main_display.set(MainDisplay::ResponseBody);

            context.set_focus("id", "response_renderer");

            let response_msg = ResponseRendererMessages::ResponseUpdate((body.to_string(), ext));
            if let Ok(msg) = serde_json::to_string(&response_msg) {
                if let Ok(component_ids) = dashboard.component_ids.try_borrow() {
                    let emitter = context.emitter.clone();
                    let _ = send_message("response_renderer", msg, component_ids, emitter);
                };
            };
        }

        Err(error) => match error {
            ureq::Error::Status(code, response) => {
                let body = response
                    .into_string()
                    .unwrap_or("Could not read error response body".to_string());
                let window_label = format!("Response Body (Status Code: {code})");

                // TODO: The error response handling needs to extract headers from the response
                // to display the response headers when there is an error

                state.response.set(body.clone());
                state.response_body_window_label.set(window_label);
                state.main_display.set(MainDisplay::ResponseBody);
                context.set_focus("id", "response_renderer");

                // TODO: Once the response headers are being extracted, figure out the correct
                // extension type to use to syntax highlight the response

                let response_msg =
                    ResponseRendererMessages::ResponseUpdate((body.to_string(), "txt".to_string()));
                if let Ok(msg) = serde_json::to_string(&response_msg) {
                    if let Ok(component_ids) = dashboard.component_ids.try_borrow() {
                        let emitter = context.emitter.clone();
                        let _ = send_message("response_renderer", msg, component_ids, emitter);
                    };
                };
            }

            ureq::Error::Transport(transport_error) => {
                let error = transport_error.message().unwrap_or("Network error");
                state.error_message.set(error.to_string());
                state.floating_window.set(FloatingWindow::Error);
            }
        },
    }
}
