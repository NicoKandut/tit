pub fn run() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    for commit_id in repository.get_commits() {
        let commit = repository.read_commit(&commit_id);
        println!("{} {}", &commit_id[0..7], commit);
    }

    for commit_id in repository.get_commits() {
        let commit = repository.read_commit(&commit_id);
        println!("{} {}", &commit_id[0..7], commit);
    }
}