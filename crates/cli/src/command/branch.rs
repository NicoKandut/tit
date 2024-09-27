use kern::Branch;

pub fn create_branch(branch_name: &str) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let mut state = repository.state();
    let commit_id = state.branches.get(&state.current.branch).unwrap().clone();
    let branch = Branch::new(branch_name.to_string(), commit_id);
    state.current.branch = branch.name.to_string();
    state.branches.insert(branch.name.clone(), branch.commit_id.clone());
    repository.set_state(state);
}