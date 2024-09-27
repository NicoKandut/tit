use kern::Branch;

pub fn create_branch(branch_name: &str) {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();
    let commit_id = state.branches.get(&state.current.branch).unwrap().clone();
    let branch = Branch::new(branch_name.to_string(), commit_id);
    state.current.branch = branch.name.to_string();
    state
        .branches
        .insert(branch.name.clone(), branch.commit_id.clone());
    repository.set_state(state);
}

pub fn list_branches() {
    let repository = kern::TitRepository::default();
    let state = repository.state();

    state
        .branches
        .iter()
        .for_each(|(name, commit_id)| println!("{} - {}", name, commit_id));
}

pub fn set_branch(branch_name: &str) {
    let repository = kern::TitRepository::default();
    let mut state = repository.state();

    if !state.branches.contains_key(branch_name) {
        println!("Branch {} not found.", branch_name);
        return;
    }

    state.current.branch = branch_name.to_string();

    repository.set_state(state);
}
