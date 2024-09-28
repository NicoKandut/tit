use parser::c;
use parser::parser::TitParser;
use parser::templater::TitTemplater;

#[test]
fn test_apply_01() {
    let src1 = r#"
        int main() {
            return 0;
        }
    "#;
    
    let src2 = r#"
        int main() {
            return 1;
        }
    "#;
    
    let mut parser = TitParser::new(c!()).unwrap();
    let mut tree1 = parser.parse(src1).unwrap();
    let tree2 = parser.parse(src2).unwrap();
    
    let changes = tree1.detect_changes(&tree2);
    tree1.apply_changes(&changes);
    
    assert_eq!(tree1, tree2);
}

#[test]
fn test_apply_02() {
    let src1 = r#"
        int main() {
        }
    "#;
    
    let src2 = r#"
        int main() {
            return 0;
        }
    "#;
    
    let mut parser = TitParser::new(c!()).unwrap();
    let mut tree1 = parser.parse(src1).unwrap();
    let tree2 = parser.parse(src2).unwrap();
    
    let changes = tree1.detect_changes(&tree2);
    tree1.apply_changes(&changes);
    
    assert_eq!(tree1, tree2);
}

#[test]
fn test_apply_03() {
    let src1 = r#"
        int main() {
            return 0;
        }
    "#;
    
    let src2 = r#"
        int main() {
        }
    "#;
    
    let mut parser = TitParser::new(c!()).unwrap();
    let mut tree1 = parser.parse(src1).unwrap();
    let tree2 = parser.parse(src2).unwrap();
    
    let changes = tree1.detect_changes(&tree2);
    tree1.apply_changes(&changes);
    
    assert_eq!(tree1, tree2);
}

#[test]
fn test_apply_04() {
    let src1 = r#"
        int main() {
            int x = 0;
            while (x < 10) {
                printf("%d\n", x);
                x++;
            }
            return 0;
        }
    "#;
    
    let src2 = r#"
        int main() {
            for (int x = 0; x < 10; x++) {
                printf("%d\n", x);
            }
            return 0;
        }
    "#;
    
    let mut parser = TitParser::new(c!()).unwrap();
    let mut tree1 = parser.parse(src1).unwrap();
    let tree2 = parser.parse(src2).unwrap();
    
    let changes = tree1.detect_changes(&tree2);
    tree1.apply_changes(&changes);
    
    assert_eq!(tree1, tree2);
}

#[test]
fn test_apply_05() {
    let src1 = r#"
        int main() {
            int x = 0;
            int y = 0;
            
            if (x == y) {
                printf("x is equal to y\n");
            } else {
                printf("x is not equal to y\n");
            }
        }
    "#;

    let src2 = r#"
        int main() {
            int x = 0;
            int y = 0;
            
            if (x == y) printf("x is equal to y\n");
            else printf("x is not equal to y\n");
        }
    "#;

    let mut parser = TitParser::new(c!()).unwrap();
    let tree1 = parser.parse(src1).unwrap();
    let tree2 = parser.parse(src2).unwrap();

    let changes = tree1.detect_changes(&tree2);
    
    assert_eq!(changes.len(), 0);
    
    println!("{:?}", tree1);
}

#[test]
fn test_render_01() {
    let src = r#"
        int main() {
            return 0;
        }
    "#;

    let mut parser = TitParser::new(c!()).unwrap();
    let tree = parser.parse(src).unwrap();

    let mut templater = TitTemplater::new(std::path::Path::new("/home/phaulson/Desktop/tit/lang/c/templates"));
    let rendered = templater.render_tree(&tree).unwrap();
    
    let expected = "int main(){return 0;}";
    assert_eq!(rendered, expected);
}

#[test]
fn test_render_02() {
    let src = r#"
        int main() {
            int x = 0;
            while (x < 10) {
                printf("x = %d\n", x);
            }
        }
    "#;

    let mut parser = TitParser::new(c!()).unwrap();
    let tree = parser.parse(src).unwrap();
    
    println!("{:?}", tree);

    let mut templater = TitTemplater::new(std::path::Path::new("/home/phaulson/Desktop/tit/lang/c/templates"));
    let rendered = templater.render_tree(&tree).unwrap();
    
    let expected = r#"int main(){int x=0;while(x<10){printf("x = %d\n",x);}}"#;
    assert_eq!(rendered, expected);
}

#[test]
fn test_render_03() {
    let src = r#"
        int main() {
            for (int x = 0; x < 10; x++) {
            }
            for (x = 0; x < 10; x++) {
            }
            for (; x < 10; x++) {
            }
            for (;; x++) {
            }
            for (;;) {
            }
        }
    "#;

    let mut parser = TitParser::new(c!()).unwrap();
    let tree = parser.parse(src).unwrap();

    let mut templater = TitTemplater::new(std::path::Path::new("/home/phaulson/Desktop/tit/lang/c/templates"));
    let rendered = templater.render_tree(&tree).unwrap();
    
    println!("{:?}", tree);

    let expected = r#"int main(){for(int x=0;x<10;x++){}for(;x<10;x++){}for(;;x++){}for(;;){}}"#;
    assert_eq!(rendered, expected);
}