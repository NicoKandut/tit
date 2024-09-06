use tree_sitter::{Node, Parser};

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_c::language()).unwrap();

    let source_code = r#"
        #include <pthread.h>

        void* my_thread(void *arg) {
            return NULL;
        }

        int main() {
            pthread_t thread;
            pthread_create(&thread, NULL, my_thread, NULL);
            return 0;
        }
    "#;
    let tree = parser.parse(source_code, None).unwrap();
    print_child_nodes(&tree.root_node(), source_code.as_bytes(), 0);
}

fn print_child_nodes(node: &Node, source: &[u8], offset: u8) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        println!(
            "{:indent$}{} -> {}",
            "",
            child.kind(),
            child.utf8_text(source).unwrap().replace("\n", ""),
            indent = offset as usize * 4
        );
        print_child_nodes(&child, source, offset + 1);
    }
}
