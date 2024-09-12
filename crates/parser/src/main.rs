mod tree;
mod errors;
mod parser;
mod macros;

use tree_sitter::Node;
use tree_sitter::Parser;
use parser::TitParser;

fn main() {
    let src_1 = r#"
    int main() {
      int x = 0;
      int y = 0;
    }
    "#;
    
    let src_2 = r#"
    int main() {
      int x = 0;
      int y = 1;
    }
    "#;

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_c::LANGUAGE.into()).unwrap();
    
    let tree1 = parser.parse(src_1, None).unwrap();
    let tree2 = parser.parse(src_2, None).unwrap();
    print_child_nodes(&tree1.root_node(), src_1.as_bytes(), 0);
    println!("-----------------");

    let mut tit_parser = TitParser::new(c!()).unwrap();
    let tit_tree1 = tit_parser.parse(src_1).unwrap();
    let tit_tree2 = tit_parser.parse(src_2).unwrap();
    let changes = tit_tree1.detect_changes(&tit_tree2).unwrap();
    
    for change in changes {
        println!("{:#?}", change);
    }
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
