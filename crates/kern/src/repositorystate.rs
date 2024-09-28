use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
};
use toml;

use crate::util::{FileRead, FileWrite};

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

impl RepositoryState {
    pub fn new(project_name: String, current_branch: String, current_server: String) -> Self {
        let mut branches = BTreeMap::new();
        branches.insert(current_branch.to_string(), "none".to_string());

        let mut servers = BTreeMap::new();
        servers.insert("default".to_string(), current_server.clone());

        Self {
            project: Project { name: project_name },
            current: Current {
                branch: branches.first_key_value().unwrap().0.clone(),
                server: servers.first_key_value().unwrap().0.clone(),
            },
            branches,
            servers,
        }
    }
}

impl<P: AsRef<Path>> FileRead<P> for RepositoryState {
    fn read_from(path: P) -> Self {
        let mut file_content = String::new();
        fs::File::open(path)
            .expect("Failed to open repository state file!")
            .read_to_string(&mut file_content)
            .expect("Failed to read file");
        toml::from_str(&file_content).expect("Failed to parse state file")
    }
}

impl<P: AsRef<Path>> FileWrite<P> for RepositoryState {
    fn write_to(&self, path: P) {
        let string = toml::to_string_pretty(self).expect("Failed to serialize state");
        fs::File::create(path)
            .expect("Failed to open repository state file!")
            .write_all(string.as_bytes())
            .expect("Failed to read file");
    }
}
