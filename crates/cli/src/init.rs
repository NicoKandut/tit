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
    fs::create_dir(core::TIT_DIR).expect("Failed to create .tit folder");

    let commits_path = Path::new(core::TIT_DIR).join(core::COMMIT_DIR);
    fs::create_dir(commits_path).expect("Failed to create commits folder");

    let repository = core::TitRepository::new(working_dir.clone());

    let commit = core::Commit::new(
        "feat: add stuff".to_string(),
        vec![
            core::Change::new(core::ChangeKind::ADDITION, "0".to_string(), "node_content".to_string()),
            core::Change::new(core::ChangeKind::ADDITION, "0,0".to_string(), "node_content_2".to_string()),
            core::Change::new(core::ChangeKind::ADDITION, "0,1".to_string(), "node_content_3".to_string()),
        ]
    );
    repository.write_commit(&commit);

    let id = commit.get_id();

    let read = repository.read_commit(&id);

    println!("Commit message: {:?}", read.message());
}
