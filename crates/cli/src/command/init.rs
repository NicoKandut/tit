use std::env::current_dir;

use crate::exitcode::{EXIT_NOT_FOUND, EXIT_OK, EXIT_UNKNOWN_RESOURCE};

pub fn init(name: Option<String>, server: Option<String>, branch: Option<String>) -> i32 {
    let working_dir = current_dir().expect("Failed to get current working directory!");

    let name = name.unwrap_or(
        working_dir
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string(),
    );
    let server = server.unwrap_or("none".to_string());
    let branch = branch.unwrap_or("main".to_string());

    let repository = kern::TitRepository::new(working_dir.clone());

    match repository.init(&name, &server, &branch) {
        Ok(_) => EXIT_OK,
        Err(err) => {
            eprintln!("ERROR: {err}");
            EXIT_UNKNOWN_RESOURCE
        }
    }
}

pub fn uninit() -> i32 {
    let working_dir = current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    match repository.uninit() {
        Ok(_) => EXIT_OK,
        Err(e) => {
            eprintln!("ERROR: {e}");
            EXIT_NOT_FOUND
        }
    }
}
