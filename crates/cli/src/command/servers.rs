use crate::exitcode::{EXIT_NOT_FOUND, EXIT_OK};

pub fn add_server(server: &str) -> i32 {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();

    eprintln!(
        "Adding remote server: {} with project: {}",
        server,
        state.project.name.clone()
    );

    state.current.server = server.to_string();

    repository.set_state(state);

    EXIT_OK
}

pub fn list_servers() -> i32 {
    let repository = kern::TitRepository::default();
    let state = repository.state();

    state
        .servers
        .iter()
        .for_each(|(name, address)| println!("{} - {}", name, address));

    EXIT_OK
}

pub fn set_server(server: &str) -> i32 {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();

    if !state.servers.contains_key(server) {
        println!("Server {} not found.", server);
        return EXIT_NOT_FOUND;
    }

    state.current.server = server.to_string();

    repository.set_state(state);

    EXIT_OK
}
