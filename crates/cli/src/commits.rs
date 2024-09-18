pub fn run() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = core::TitRepository::new(working_dir);
    repository.read_all_commits();
}