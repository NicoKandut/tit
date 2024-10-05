use crate::util::TomlFile;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Current {
    pub branch: String,
    pub server: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryState {
    pub project: Project,
    pub current: Current,
    pub branches: BTreeMap<String, String>,
    pub servers: BTreeMap<String, String>,
}

impl TomlFile for RepositoryState {}

impl RepositoryState {
    pub fn new(project_name: String, current_branch: String, current_server: String) -> Self {
        // prepare branches
        let mut branches = BTreeMap::new();
        branches.insert(current_branch.to_string(), "none".to_string());

        // prepare servers
        let mut servers = BTreeMap::new();
        servers.insert("default".to_string(), current_server.clone());

        // prepare project
        let project = Project { name: project_name };

        // prepare current state
        let current = Current {
            branch: branches
                .first_key_value()
                .expect("Cannot get first branch")
                .0
                .clone(),
            server: servers
                .first_key_value()
                .expect("Cannot get first server")
                .0
                .clone(),
        };

        Self {
            project,
            current,
            branches,
            servers,
        }
    }
}
