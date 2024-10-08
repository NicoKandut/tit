use crate::{hashtree::HashTree, ignore::get_ignorelist_of_dir, Node};
use std::path::Path;

const KIND_DIR: &str = "dir";
const KIND_FILE: &str = "file";

pub fn build_hash_tree_for_dir(root_dir: &Path) -> HashTree<Node> {
    let mut tree = HashTree::default();
    scan_and_add_fs_entry(&mut tree, None, root_dir);
    tree
}

fn scan_and_add_fs_entry(arena: &mut HashTree<Node>, parent: Option<usize>, path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    if path.is_dir() {
        let dir_node = Node {
            kind: KIND_DIR.to_string(),
            value: Some(name.clone()),
            role: None,
        };
        let new_node_id = match parent {
            Some(parent) => arena
                .insert(parent, dir_node)
                .expect("Failed to insert node"),
            None => arena.insert_root(dir_node),
        };

        let ignored = get_ignorelist_of_dir(path);
        let included = path
            .read_dir()
            .unwrap()
            .filter_map(|e| match e {
                Ok(e) => Some(e),
                Err(_) => None,
            })
            .map(|e| e.path())
            .filter(|e| !ignored.contains(&e.file_name().unwrap().to_str().unwrap().to_string()))
            .collect::<Vec<_>>();

        for entry in included {
            scan_and_add_fs_entry(arena, Some(new_node_id), &entry);
        }
    } else {
        let content = "Load real content as AST later...".to_string();
        let file_node = Node {
            kind: KIND_FILE.to_string(),
            value: Some(name.clone()),
            role: Some(content),
        };

        arena
            .insert(
                parent.expect("Cannot insert file without parent!"),
                file_node,
            )
            .expect("Failed to insert node");
    }
}
