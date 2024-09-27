pub fn list_commits() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);

    repository
        .commits()
        .values()
        .for_each(|commit| println!("{}", commit));
}
