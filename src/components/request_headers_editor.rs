use anathema::{
    component::Component,
    state::{CommonVal, List, State, Value},
};

pub const REQUEST_HEADERS_EDITOR_TEMPLATE: &str =
    "./src/components/templates/request_headers_editor.aml";

#[derive(Default)]
pub struct RequestHeadersEditor;

#[derive(Default, State)]
pub struct Header {
    pub name: Value<String>,
    pub value: Value<String>,
}

impl From<CommonVal<'_>> for Header {
    fn from(value: CommonVal<'_>) -> Self {
        println!("From<CommonVal> for Header: 1");
        let str = value.to_string();
        println!("From<CommonVal> for Header: 2");
        let Some((name, value)) = str.split_once("__") else {
            println!("From<CommonVal> for Header: 3");
            return Header {
                name: "".to_string().into(),
                value: "".to_string().into(),
            };
        };

        println!("From<CommonVal> for Header: 4");
        Header {
            name: name.to_string().into(),
            value: value.to_string().into(),
        }
    }
}

impl From<Header> for String {
    fn from(value: Header) -> Self {
        println!("From<Header> for Header: 1");
        format!("{}__{}", *value.name.to_ref(), *value.value.to_ref())
    }
}

// impl anathema::state::State for MainDisplay {
//     fn to_common(&self) -> Option<CommonVal<'_>> {
//         match self {
//             MainDisplay::RequestBoy => Some(CommonVal::Str("request_body")),
//             MainDisplay::RequestHeadersEditor => Some(CommonVal::Str("request_headers_editor")),
//             MainDisplay::ResponseBody => Some(CommonVal::Str("response_body")),
//         }
//     }
// }

#[derive(Default, State)]
pub struct RequestHeadersEditorState {
    // headers: Value<List<Header>>,
}

impl RequestHeadersEditorState {
    pub fn new() -> Self {
        RequestHeadersEditorState {
            // headers: List::from_iter(get_default_headers()),
        }
    }
}

impl Component for RequestHeadersEditor {
    type State = RequestHeadersEditorState;
    type Message = ();
}
