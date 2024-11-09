use serde::{Deserialize, Serialize};

use crate::projects::PersistedProject;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmDeleteProject {
    pub project: PersistedProject,
    pub title: String,
    pub message: String,
}

impl ComponentMessage for ConfirmDeleteProject {
    fn get_message_type(&self) -> String {
        String::from("confirm_delete_project")
    }
}

pub trait ComponentMessage {
    fn get_message_type(&self) -> String;
}
