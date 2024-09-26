pub fn init() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    match repository.init() {
        Ok(_) => println!("Successful!"),
        Err(err) => println!("ERROR: {err}"),
    }
}
