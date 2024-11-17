use anathema::state::{List, State, Value};
use serde::{Deserialize, Serialize};
use std::{fs, ops::Deref};

use crate::fs::get_app_dir;

// TODO: Fix the default project row color to the correct gray
const DEFAULT_ROW_COLOR: &str = "#333333";

// TODO: Implement using this constant for selected rows
#[allow(unused)]
const SELECTED_ROW_COLOR: &str = "#FFFFFF";

pub const DEFAULT_PROJECT_NAME: &str = "Unnamed";
pub const DEFAULT_ENDPOINT_NAME: &str = "Unnamed";

#[derive(anathema::state::State)]
pub struct Project {
    pub name: Value<String>,
    pub endpoints: Value<List<Endpoint>>,
    pub row_color: Value<String>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            name: String::from(DEFAULT_PROJECT_NAME).into(),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
            endpoints: List::empty(),
        }
    }
}

#[derive(anathema::state::State)]
pub struct Endpoint {
    pub name: Value<String>,
    pub url: Value<String>,
    pub method: Value<String>,
    pub headers: Value<List<HeaderState>>,
    pub body: Value<String>,
    pub row_color: Value<String>,
}

impl Endpoint {
    pub fn new() -> Self {
        Endpoint {
            name: String::from(DEFAULT_ENDPOINT_NAME).into(),
            url: String::from("").into(),
            method: String::from("GET").into(),
            body: String::from("").into(),
            headers: List::from_iter(get_default_headers()),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
        }
    }

    pub fn clone(&self) -> Self {
        let headers_list = self.headers.to_ref();
        let headers = headers_list.iter().map(|header| {
            let h = header.to_ref();
            h.clone()
        });

        Endpoint {
            name: self.name.to_ref().to_string().into(),
            url: self.url.to_ref().to_string().into(),
            method: self.method.to_ref().to_string().into(),
            body: self.body.to_ref().to_string().into(),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
            headers: List::from_iter(headers),
        }
    }
}

#[derive(Debug, Default, State)]
pub struct HeaderState {
    pub name: Value<String>,
    pub value: Value<String>,
}

impl HeaderState {
    pub fn clone(&self) -> Self {
        HeaderState {
            name: self.name.to_ref().to_string().into(),
            value: self.value.to_ref().to_string().into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersistedProject {
    pub name: String,
    pub endpoints: Vec<PersistedEndpoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersistedEndpoint {
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: Vec<Header>,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
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

pub fn save_project(project: PersistedProject) -> anyhow::Result<()> {
    if project.name.trim() == "" {
        return Err(anyhow::Error::msg("Project must have name"));
    }

    let serialization_result = serde_json::to_string(&project);

    if serialization_result.is_err() {
        return Err(anyhow::Error::msg("Unable to serialize project"));
    }

    let dir_result = get_app_dir("projects");
    if dir_result.is_err() {
        return Err(anyhow::Error::msg("Unable to access projects directory"));
    }

    let mut project_dir = dir_result.unwrap();
    let serialized_project = serialization_result.unwrap();
    project_dir.push(format!("{}.project", project.name));

    let write_result = fs::write(project_dir, serialized_project);
    if write_result.is_err() {
        let write_error = write_result.unwrap_err();

        return Err(anyhow::Error::msg(write_error.to_string()));
    }

    Ok(())
}

pub fn get_projects() -> anyhow::Result<Vec<PersistedProject>> {
    let dir_result = get_app_dir("projects");
    if dir_result.is_err() {
        return Err(anyhow::Error::msg("Unable to access projects directory"));
    }

    let project_dir = dir_result.unwrap();

    let read_dir = fs::read_dir(project_dir)?;

    Ok(read_dir
        .flatten()
        .flat_map(|entry| fs::read_to_string(entry.path()))
        .flat_map(|content| serde_json::from_str::<PersistedProject>(&content))
        .collect::<Vec<PersistedProject>>())
}

#[allow(unused)]
pub fn get_project_list() -> anyhow::Result<Value<List<Project>>> {
    match get_projects() {
        Ok(projects) => Ok(List::<Project>::from_iter(
            projects.iter().map(|project| project.into()),
        )),
        Err(error) => Err(error),
    }
}

impl From<&Endpoint> for PersistedEndpoint {
    fn from(endpoint: &Endpoint) -> Self {
        let mut headers: Vec<Header> = vec![];

        endpoint.headers.to_ref().iter().for_each(|header_state| {
            let h_state = header_state.to_ref();
            headers.push(h_state.deref().into());
        });

        PersistedEndpoint {
            name: endpoint.name.to_ref().to_string(),
            url: endpoint.url.to_ref().to_string(),
            method: endpoint.method.to_ref().to_string(),
            body: endpoint.body.to_ref().to_string(),
            headers,
        }
    }
}

impl From<&Project> for PersistedProject {
    fn from(project: &Project) -> Self {
        let mut endpoints: Vec<PersistedEndpoint> = vec![];
        project
            .endpoints
            .to_ref()
            .iter()
            .for_each(|endpoint_value| {
                let endpoint = endpoint_value.to_ref();
                endpoints.push(endpoint.deref().into());
            });

        let name = project.name.to_ref().clone();
        PersistedProject { name, endpoints }
    }
}

impl From<&HeaderState> for Header {
    fn from(header_state: &HeaderState) -> Self {
        Header {
            name: header_state.name.to_ref().to_string(),
            value: header_state.value.to_ref().to_string(),
        }
    }
}

impl From<&PersistedProject> for Project {
    fn from(persisted_project: &PersistedProject) -> Self {
        let endpoints: Value<List<Endpoint>> = List::from_iter(
            persisted_project
                .endpoints
                .iter()
                .map(|persisted_endpoint| persisted_endpoint.into()),
        );

        Project {
            name: persisted_project.name.clone().into(),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
            endpoints,
        }
    }
}

impl From<&PersistedEndpoint> for Endpoint {
    fn from(persisted_endpoint: &PersistedEndpoint) -> Self {
        let headers: Value<List<HeaderState>> = List::from_iter(
            persisted_endpoint
                .headers
                .iter()
                .map(|header| header.into()),
        );

        Endpoint {
            name: persisted_endpoint.name.clone().into(),
            body: persisted_endpoint.body.clone().into(),
            url: persisted_endpoint.url.clone().into(),
            method: persisted_endpoint.method.clone().into(),
            row_color: DEFAULT_ROW_COLOR.to_string().into(),
            headers,
        }
    }
}

impl From<&Header> for HeaderState {
    fn from(header: &Header) -> Self {
        HeaderState {
            name: header.name.clone().into(),
            value: header.value.clone().into(),
        }
    }
}
