use kern::TitRepository;

use crate::exitcode::EXIT_OK;

pub fn list_changes() -> i32 {
    let repository = TitRepository::default();
    let before = repository.signed_tree();
    let after = repository.current_tree();
    let difference = before.difference(&after);

    for change in difference {
        println!("{}", change);
    }

    EXIT_OK
}
