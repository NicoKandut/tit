use parser::TitParser;

mod tree;
mod errors;
mod parser;
mod macros;

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
    
    let mut tit_parser = TitParser::new(c!()).unwrap();
    let tit_tree1 = tit_parser.parse(src_1).unwrap();
    let tit_tree2 = tit_parser.parse(src_2).unwrap();
    let changes = tit_tree1.detect_changes(&tit_tree2).unwrap();

    tit_tree1.print();
    println!("-----------------");
    
    for change in changes {
        println!("{:#?}", change);
    }
}
