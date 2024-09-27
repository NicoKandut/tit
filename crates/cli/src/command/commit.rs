use kern::{util::get_epoch_millis, Node};

pub fn commit(message: String) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);

    // Test commit handling
    let commit = kern::Commit::new(
        message,
        vec![
            kern::Change::Addition(
                [0].to_vec(),
                Node {
                    kind: "node_kind_1".to_string(),
                    value: Some("node_content_1".to_string()),
                    role: None,
                },
            ),
            kern::Change::Addition(
                [0, 0].to_vec(),
                Node {
                    kind: "node_kind_2".to_string(),
                    value: Some("node_content_2".to_string()),
                    role: None,
                },
            ),
            kern::Change::Addition(
                [0, 1].to_vec(),
                Node {
                    kind: "node_kind_3".to_string(),
                    value: Some("node_content_3".to_string()),
                    role: None,
                },
            ),
        ],
        get_epoch_millis(),
        None,
    );
    repository.write_commit(&commit);
    println!("Committing: {}", commit);

    let mut state = repository.state();
    state.branches.insert(state.current.branch.clone(), commit.get_id());
    repository.set_state(state);
}
