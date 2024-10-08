use std::{fs, path::Path};

use crate::{DOT_GIT, DOT_TIT, DOT_TIT_IGNORE};

pub fn get_ignorelist_of_dir(path: &Path) -> Vec<String> {
    let ignore_file = path.join(DOT_TIT_IGNORE);
    let mut ignore_list = vec![DOT_TIT.to_string(), DOT_GIT.to_string()];

    if ignore_file.exists() {
        let manually_ignored = read_ignore_file(&ignore_file);
        ignore_list.extend(manually_ignored);
    }

    ignore_list
}

fn read_ignore_file(path: &Path) -> Vec<String> {
    let contents =
        fs::read_to_string(path).expect(&format!("Failed to read ignore file at {:?}", path));
    contents
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<_>>()
}
