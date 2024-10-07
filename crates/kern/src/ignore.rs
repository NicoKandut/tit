use std::{fs, path::Path};

pub fn get_ignorelist_of_dir(path: &Path) -> Vec<String> {
    let ignore_file = path.join(".titignore");
    let mut ignore_list = vec![".tit".to_string(), ".git".to_string()];

    if ignore_file.exists() {
        let manually_ignored = read_ignore_file(&ignore_file);
        ignore_list.extend(manually_ignored);
    }

    ignore_list
}

fn read_ignore_file(path: &Path) -> Vec<String> {
    let error_msg = format!("Failed to read ignore file at {:?}", path);
    let contents = fs::read_to_string(path).expect(&error_msg);
    contents.lines().map(|l| l.to_string()).collect::<Vec<_>>()
}
