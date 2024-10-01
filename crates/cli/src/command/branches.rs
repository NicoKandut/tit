use kern::Branch;

use crate::exitcode::{EXIT_NOT_FOUND, EXIT_OK};

pub fn create_branch(branch_name: &str) -> i32 {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();
    let commit_id = match state.branches.get(&state.current.branch) {
        Some(commit_id) => commit_id.clone(),
        None => {
            eprintln!("Current branch not found.");
            return EXIT_NOT_FOUND;
        }
    };
    let branch = Branch::new(branch_name.to_string(), commit_id);
    state.current.branch = branch.name.to_string();
    state
        .branches
        .insert(branch.name.clone(), branch.commit_id.clone());
    repository.set_state(state);

    EXIT_OK
}

pub fn list_branches() -> i32 {
    let repository = kern::TitRepository::default();
    let state = repository.state();

    state
        .branches
        .iter()
        .for_each(|(name, commit_id)| println!("{} - {}", name, commit_id));

    EXIT_OK
}

pub fn set_branch(branch_name: &str) -> i32 {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();

    if !state.branches.contains_key(branch_name) {
        eprintln!("Branch {} not found.", branch_name);
        return EXIT_NOT_FOUND;
    }

    state.current.branch = branch_name.to_string();

    repository.set_state(state);

    EXIT_OK
}
