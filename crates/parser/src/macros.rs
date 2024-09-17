#[macro_export]
macro_rules! c {
    () => {
        tree_sitter_c::LANGUAGE.into()
    };
}