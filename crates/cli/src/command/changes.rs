use kern::TitRepository;

pub fn list_changes() {
    let repository = TitRepository::default();
    let before = repository.signed_tree();
    let after = repository.current_tree();
    let difference = before.difference(&after);

    for change in difference {
        println!("{}", change);
    }
}
