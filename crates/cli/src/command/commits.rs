use crate::exitcode::EXIT_OK;

pub fn list_commits() -> i32 {
    let repository = kern::TitRepository::default();

    repository
        .commits()
        .values()
        .for_each(|commit| println!("{}", commit));

    EXIT_OK
}
