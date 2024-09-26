use kern::Branch;

pub fn create_branch(branch_name: &str) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let mut state = repository.state();
    let branch = Branch::new(branch_name.to_string(), state.current_commit.clone());
    repository.write_branch(&branch);
    state.current_branch = branch.name.to_string();
    repository.set_state(state);
}