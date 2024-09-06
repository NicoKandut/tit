mod tree;
mod errors;
mod parser;

use tree_sitter::{Node};
use crate::parser::TitParser;

fn main() {
    let src_1 = r#"
    int main() {
      char* x = "hello";
    }
    "#;

    let src_2 = r#"
    int main() {
      int x = 0;
    }
    "#;
    
    let mut parser = TitParser::new(&tree_sitter_c::language()).unwrap();
    let tree = parser.parse(src_1).unwrap();
}

fn print_child_nodes(node: &Node, source: &[u8], offset: u8) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        println!(
            "{:indent$}{} -> {}",
            "",
            child.kind(),
            child.utf8_text(source).unwrap().replace("\n", ""),
            indent = offset as usize * 2
        );
        print_child_nodes(&child, source, offset + 1);
    }
}

fn detect_tree_differences(
    n1: &Node,
    n2: &Node,
    src_1: &[u8],
    src_2: &[u8],
    path: &str,
    offset: usize,
) -> Vec<(usize, String)> {
    let mut differences = Vec::new();

    if n1.named_child_count() > n2.named_child_count() {
        for i in n2.named_child_count()..n1.named_child_count() {
            differences.push((
                offset + 1,
                format!("Deletion at {}/{}", path, n1.named_child(i).unwrap().kind()),
            ));
        }
    } else if n2.named_child_count() > n1.named_child_count() {
        for i in n1.named_child_count()..n2.named_child_count() {
            differences.push((
                offset + 1,
                format!(
                    "Insertion at {}/{}",
                    path,
                    n2.named_child(i).unwrap().kind()
                ),
            ));
        }
    }

    if n1.kind() != n2.kind() {
        differences.push((
            offset,
            format!("Kind mismatch at {}: {} vs {}", path, n1.kind(), n2.kind()),
        ));
    }

    if n1.child_count() == 0 && n2.child_count() == 0 && n1.utf8_text(src_1) != n2.utf8_text(src_2)
    {
        differences.push((
            offset,
            format!(
                "Content mismatch at {}: {:?} vs {:?}",
                path,
                n1.utf8_text(src_1).unwrap(),
                n2.utf8_text(src_2).unwrap()
            ),
        ));
    }

    for i in 0..usize::min(n1.named_child_count(), n2.named_child_count()) {
        let new_path = format!("{}/{}", path, n1.named_child(i).unwrap().kind());
        let child_diffs = detect_tree_differences(
            &n1.named_child(i).unwrap(),
            &n2.named_child(i).unwrap(),
            src_1,
            src_2,
            &new_path,
            offset + 1,
        );
        differences.extend(child_diffs);
    }

    differences
}
