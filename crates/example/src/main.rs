use std::path::PathBuf;
use parser::c;
use parser::parser::TitParser;
use templater::TitTemplater;

fn main() {
    let src = r#"
        int main(int argc, char **argv) {
            int* a = 0;
            a.b = 5;
        }
    "#;

    let mut parser = TitParser::new(c!()).unwrap();
    let tree = parser.parse(src).unwrap();
    
    println!("{:?}", tree);

    let mut templater = TitTemplater::new(PathBuf::from("/home/phaulson/Desktop/tit/lang/c/templates"));
    let rendered = templater.render_tree(&tree).unwrap();
    println!("{:?}", rendered);
}
