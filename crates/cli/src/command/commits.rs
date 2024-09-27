pub fn list_commits() {
    let repository = kern::TitRepository::default();

    repository
        .commits()
        .values()
        .for_each(|commit| println!("{}", commit));
}
