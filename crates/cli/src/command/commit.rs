use kern::util::get_epoch_millis;

use crate::exitcode::EXIT_OK;

pub fn commit(message: String) -> i32 {
    let repository = kern::TitRepository::default();

    let before = repository.signed_tree();
    let after = repository.current_tree();
    let difference = before.difference(&after);

    // Test commit handling
    let commit = kern::Commit::new(message, difference, get_epoch_millis(), None);
    repository.write_commit(&commit);
    println!("Committing: {}", commit);

    let mut state = repository.state();
    state
        .branches
        .insert(state.current.branch.clone(), commit.get_id());
    repository.set_state(state);
    repository.set_signed_tree(after);

    EXIT_OK
}
