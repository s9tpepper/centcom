use serde::{Deserialize, Serialize};

use crate::projects::PersistedEndpoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmDeleteEndpoint {
    pub endpoint: PersistedEndpoint,
    pub title: String,
    pub message: String,
}

impl ComponentMessage for ConfirmDeleteEndpoint {
    fn get_message_type(&self) -> String {
        String::from("confirm_delete_endpoint")
    }
}

pub trait ComponentMessage {
    #[allow(unused)]
    fn get_message_type(&self) -> String;
}
