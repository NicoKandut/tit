pub fn run() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    
    repository
        .get_commits()
        .iter()
        .map(|id| repository.read_commit(id))
        .for_each(|commit| println!("{}", commit));
}
