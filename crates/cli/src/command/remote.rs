pub fn add_server(server: &str) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let mut state = repository.state();

    println!("Adding remote server: {} with project: {}", server, state.project.name.clone());

    state.current.server = server.to_string();

    repository.set_state(state);
}
