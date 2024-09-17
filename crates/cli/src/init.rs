use core::change::{Change, ChangeKind};
use core::{commit::Commit, repository::TitRepository, COMMIT_DIR, TIT_DIR};
use std::fs;
use std::path::Path;

pub fn run() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");

    let tit_exists = fs::read_dir(working_dir.clone())
        .expect("Failed to read entries of cwd")
        .any(|entry| entry.expect("Failed to read entry").file_name() == core::TIT_DIR);

    if tit_exists {
        println!("Repository already initialized!");
        return;
    }

    println!("Initializing project...");
    fs::create_dir(TIT_DIR).expect("Failed to create .tit folder");

    let commits_path = Path::new(TIT_DIR).join(COMMIT_DIR);
    fs::create_dir(commits_path).expect("Failed to create commits folder");

    let repository = TitRepository::new(working_dir.clone());

    let commit = Commit::new(
        "feat: add stuff".to_string(),
        vec![
            Change {
                path: vec![0, 1, 1, 2],
                kind: ChangeKind::Addition {
                    kind: "primitive_type".to_string(),
                    value: Some("int".to_string()),
                },
            },
            Change { path: vec![0, 1], kind: ChangeKind::Deletion },
            Change {
                path: vec![0, 1, 2, 2, 2],
                kind: ChangeKind::ValueUpdate("int".to_string())
            },
        ],
    );
    repository.write_commit(&commit);

    let id = commit.get_id();

    let read = repository.read_commit(&id);

    println!("Commit message: {:?}", read.message());
}
