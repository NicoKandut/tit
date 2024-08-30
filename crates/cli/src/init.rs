use std::fs;

const METADATA_DIR: &str = ".tit";

pub fn run() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");

    let tit_exists = fs::read_dir(working_dir)
        .expect("Failed to read entries of cwd")
        .any(|entry| entry.expect("Failed to read entry").file_name() == METADATA_DIR);

    if tit_exists {
        println!("Repository already initialized!");
        return;
    }

    println!("Initializing project...");
    fs::create_dir(METADATA_DIR).expect("Failed to create .tit folder")
}
