use anathema::{
    component::Component,
    state::{State, Value},
};

pub const REQUEST_HEADERS_EDITOR_TEMPLATE: &str =
    "./src/components/templates/request_headers_editor.aml";

#[derive(Default)]
pub struct RequestHeadersEditor;

#[derive(Debug, Default, State)]
pub struct HeaderState {
    pub name: Value<String>,
    pub value: Value<String>,
}

#[derive(Default, State)]
pub struct RequestHeadersEditorState {}

impl RequestHeadersEditorState {
    pub fn new() -> Self {
        RequestHeadersEditorState {}
    }
}

impl Component for RequestHeadersEditor {
    type State = RequestHeadersEditorState;
    type Message = ();
}
