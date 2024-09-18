use std::time::{Duration, SystemTime};

pub fn run(message: String) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = core::TitRepository::new(working_dir);
    // Test commit handling
    let commit = core::Commit::new(
        message,
        vec![
            core::Change::new(core::ChangeKind::ADDITION, "0".to_string(), "node_content".to_string()),
            core::Change::new(core::ChangeKind::ADDITION, "0,0".to_string(), "node_content_2".to_string()),
            core::Change::new(core::ChangeKind::ADDITION, "0,1".to_string(), "node_content_3".to_string()),
        ],
        core::get_epoch_millis()
    );
    repository.write_commit(&commit);
    let id = commit.get_id();
    let read = repository.read_commit(&id);
    println!("Commit message: {:?}", read.message());
}