use crate::c;
use std::collections::{HashMap, HashSet};
use tree_sitter::{Language, LanguageRef};

pub type Kinds = HashSet<String>;

macro_rules! string_set {
    ($($item:expr),*) => {{
        use std::collections::HashSet;
        let mut set = HashSet::new();
        $(set.insert($item.to_string());)*
        set
    }};
    () => {};
}

pub fn significant_unnamed_kinds(lang: LanguageRef) -> Kinds {
    let language_sets: HashMap<Language, Kinds> = HashMap::from_iter([(
        c!(),
        string_set!(
            "*",
            "/",
            "+",
            "-",
            "%",
            "&",
            "|",
            "<",
            ">",
            "*=",
            "/=",
            "+=",
            "-=",
            "%=",
            "&=",
            "|=",
            "<<",
            ">>",
            ">>=",
            "<<=",
            "&&",
            "||",
            "!",
            "~",
            "^",
            "^=",
            "==",
            "!=",
            "<=",
            ">=",
            "++",
            "--",
            ".",
            "->"
        ),
    )]);

    language_sets.get(&*lang).unwrap().clone()
}

pub fn insignificant_named_kinds(lang: LanguageRef) -> Kinds {
    let language_sets: HashMap<Language, Kinds> = HashMap::from_iter([(
        c!(),
        string_set!(
           "parenthesized_expression",
            "compound_statement"
        ),
    )]);

    language_sets.get(&*lang).unwrap().clone()
}
