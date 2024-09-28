use std::collections::HashMap;
use std::{fs, io};
use std::path::Path;
use kern::Node;
use crate::errors::RenderingError;
use crate::templater::replacement::Replacement;
use crate::templater::template::{ChildDelimiters, FALL_BACK_TEMPLATE_CHILDREN, FALL_BACK_TEMPLATE_VALUE, Template, TEMPLATE_PATTERN, TEMPLATE_PATTERN_CHILDREN};
use kern::TitTree;

mod template;
mod replacement;

type Templates = HashMap<String, Template>;
type Replacements = HashMap<String, Replacement>;

pub struct TitTemplater<'a> {
    folder: &'a Path,
    template_regex: regex::Regex,
    children_template_regex: regex::Regex,
    templates: Templates,
}

impl<'a> TitTemplater<'a> {
    pub fn new(folder: &'a Path) -> Self {
        TitTemplater {
            folder,
            template_regex: regex::Regex::new(TEMPLATE_PATTERN).expect("Regex pattern should be valid"),
            children_template_regex: regex::Regex::new(TEMPLATE_PATTERN_CHILDREN).expect("Regex pattern should be valid"),
            templates: Templates::new(),
        }
    }
    
    // pub fn render_tree(&mut self, tree: &TitTree) -> Result<String, RenderingError> {
    //     let root = tree.root().map_err(RenderingError::TreeIteratingError)?;
    //     self.render_node(&root, tree)
    // }
    // 
    // fn render_node(
    //     &mut self,
    //     node: &indextree::Node<Node>,
    //     tree: &TitTree,
    // ) -> Result<String, RenderingError> {
    //     let node_value = node.get();
    //     
    //     let template = match self.templates.get(&node_value.kind) {
    //         Some(template) => Some(template),
    //         None => {
    //             let new_template = self.create_template_from_node(&node_value)
    //                 .map_err(RenderingError::IoError)?;
    // 
    //             if let Some(template) = new_template {
    //                 self.templates.insert(node_value.kind.clone(), template);
    //                 self.templates.get(&node_value.kind)
    //             } else {
    //                 None
    //             }
    //         }
    //     };
    // 
    //     let template_content = match template {
    //         Some(template) => &template.content,
    //         None => match &node_value.value {
    //             Some(_) => FALL_BACK_TEMPLATE_VALUE,
    //             None => FALL_BACK_TEMPLATE_CHILDREN
    //         }
    //     };
    //     let children_delimiters = match template {
    //         Some(template) => &template.children_delimiters.clone(),
    //         None => &ChildDelimiters::from([(FALL_BACK_TEMPLATE_CHILDREN.to_string(), "".to_string())])
    //     };
    // 
    //     let mut result = template_content.to_string();
    //     let mut replacements = Replacements::new();
    // 
    //     if let Some(value) = &node_value.value {
    //         insert_content(&mut replacements, FALL_BACK_TEMPLATE_VALUE, value.clone());
    //     }
    // 
    //     for (index, child) in tree.children(node).map_err(RenderingError::TreeIteratingError)?.iter().enumerate() {
    //         let child_result = self.render_node(&child, tree)?;
    // 
    //         if let Some(role) = &child.get().role {
    //             let role_placeholder = format!("${}$", role);
    //             if result.contains(&role_placeholder) {
    //                 insert_content(&mut replacements, &role_placeholder, child_result.to_string());
    //             }
    //         } else {
    //             let index_placeholder = format!("${}$", index);
    //             if result.contains(&index_placeholder) {
    //                 insert_content(&mut replacements, &index_placeholder, child_result.to_string());
    //             } else {
    //                 for (children_placeholder, delimiter) in children_delimiters {
    //                     insert_content_with_delimiter(&mut replacements, children_placeholder, child_result.to_string(), delimiter);
    //                 }
    //             }
    //         }
    //     }
    // 
    //     for (placeholder, content) in replacements {
    //         result = result.replace(&placeholder, &content.content.join(content.delimiter.as_str()));
    //     }
    //     
    //     result = self.template_regex.replace_all(&result, "").to_string();
    //     Ok(result)
    // }
    // 
    // fn create_template_from_node(&self, node: &Node) -> Result<Option<Template>, io::Error> {
    //     let path = self.folder.join(&node.kind);
    //     if !path.exists() {
    //         return Ok(None);
    //     }
    // 
    //     let content = fs::read_to_string(path)?;
    //     let mut children_delimiters = ChildDelimiters::new();
    //     for captures in self.children_template_regex.captures_iter(content.as_str()) {
    //         if let Some(full_match) = captures.get(0) {
    //             let part_after_colon = captures.get(1).map_or("", |m| m.as_str());
    //             children_delimiters.insert(full_match.as_str().to_string(), part_after_colon.to_string());
    //         }
    //     }
    //     
    //     Ok(Some(Template {
    //         name: node.kind.clone(),
    //         content,
    //         children_delimiters
    //     }))
    // }
}

fn insert_content(replacements: &mut Replacements, key: &str, value: String) {
    insert_content_with_delimiter(replacements, key, value, "");
}

fn insert_content_with_delimiter(replacements: &mut Replacements, key: &str, value: String, delimiter: &str) {
    replacements.entry(key.to_string()).or_insert(Replacement {
        placeholder: key.to_string(),
        content: vec![],
        delimiter: delimiter.to_string(),
    }).content.push(value);
}