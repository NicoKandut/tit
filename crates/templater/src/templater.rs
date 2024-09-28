use std::collections::HashMap;
use std::path::PathBuf;

use kern::{Node, TitError, TitTree};

use crate::template::Template;

pub struct TitTemplater {
    folder: PathBuf,
    templates: HashMap<String, Template>,
}

impl TitTemplater {
    pub fn new(folder: PathBuf) -> Self {
        Self {
            folder,
            templates: HashMap::new(),
        }
    }

    pub fn render_tree(&mut self, tree: &TitTree) -> Result<String, TitError> {
        let root = tree.root()?;
        self.render_node(&root, tree)
    }

    fn render_node(
        &mut self,
        node: &indextree::Node<Node>,
        tree: &TitTree,
    ) -> Result<String, TitError<'static>> {
        let node_value = node.get();

        let template = match self.templates.get(&node_value.kind) {
            Some(template) => template.clone(),
            None => {
                let new_template = Template::from_path(&self.folder, &node_value.kind);
                match new_template {
                    Ok(template) => {
                        self.templates.insert(node_value.kind.clone(), template);
                        self.templates
                            .get(&node_value.kind)
                            .expect("Template should exist")
                            .clone()
                    }
                    Err(_) if node_value.value.is_some() => Template::value_default(),
                    _ => Template::children_default()
                }
            }
        };

        let mut replacements = HashMap::new();
        
        if let Some(value) = &node_value.value {
            insert_content(&mut replacements, "__value__", value.clone(), &node_value.kind);
        }
        
        for child in tree.children(node)? {
            let child_value = child.get();
            let child_result = self.render_node(&child, tree)?;

            if let Some(role) = &child.get().role {
                insert_content(&mut replacements, role, child_result, &child_value.kind);
            } else {
                insert_content(&mut replacements, "__children__", child_result, &child_value.kind);
            }
        }

        Ok(template.render(&replacements))
    }
}

fn insert_content<'a>(replacements: &mut HashMap<&'a str, (Vec<String>, &'a str)>, key: &'a str, value: String, kind: &'a str) {
    let (values, _) = replacements.entry(key).or_insert((Vec::new(), kind));
    values.push(value);
}
