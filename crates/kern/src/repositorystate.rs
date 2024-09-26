use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::Path,
};
use toml;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryState {
    pub project_name: String,
    pub current_branch: String,
    pub current_server: String,
    pub current_commit: String,
}

impl RepositoryState {
    pub fn new(
        project_name: String,
        current_branch: String,
        current_server: String,
        current_commit: String,
    ) -> Self {
        Self {
            project_name,
            current_branch,
            current_server,
            current_commit,
        }
    }

    pub fn load(path: &Path) -> Self {
        let mut file_content = String::new();
        fs::File::open(path)
            .expect("Failed to open repository state file!")
            .read_to_string(&mut file_content)
            .expect("Failed to read file");

        toml::from_str(&file_content).expect("Failed to parse state file")
    }

    pub fn save(path: &Path, state: RepositoryState) {
        let string = toml::to_string_pretty(&state).expect("Failed to serialize state");

       fs::File::create(path)
            .expect("Failed to open repository state file!")
            .write_all(string.as_bytes())
            .expect("Failed to read file");
    }
}
