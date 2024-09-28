use crate::palceholder::Placeholder;
use kern::TitError;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

lazy_static! {
    static ref PLACEHOLDER_REGEX: Regex = Regex::new(r"\$(?P<placeholder>(?P<role>[a-zA-Z_]+)(\[(?P<prefix>prefix) ?(?P<prefix_not>!?)(?P<prefix_key>[a-zA-Z_]+)=(?P<prefix_value>.+?)])?(\[(?P<suffix>suffix) ?(?P<suffix_not>!?)(?P<suffix_key>[a-zA-Z_]+)=(?P<suffix_value>.+?)])?(\[(?P<delim>delim) ?(?P<delim_value>.+?)])?)\$").expect("Regex should be valid");
}

#[derive(Clone)]
pub struct Template {
    pub content: String,
    pub placeholders: Vec<Placeholder>,
}

impl Template {
    pub fn from_path(folder: &Path, name: &str) -> Result<Template, TitError<'static>> {
        let path = folder.join(name);
        if !path.exists() {
            return Err(TitError("Template file does not exist", None));
        }

        let mut placeholders = Vec::new();
        let content = std::fs::read_to_string(&path)
            .map_err(|e| TitError("Failed to read template file", Some(e)))?;
        for caps in PLACEHOLDER_REGEX.captures_iter(&content) {
            placeholders.push(Placeholder::from_caps(&caps));
        }

        let template = Template {
            content,
            placeholders,
        };

        Ok(template)
    }

    pub fn render(&self, replacements: &HashMap<&str, (Vec<String>, &str)>) -> String {
        let mut result = self.content.clone();
        let empty_replacement = (Vec::new(), "");

        for placeholder in &self.placeholders {
            let (values, kind) = replacements.get(&placeholder.name.as_str()).unwrap_or(&empty_replacement);
            let rendered = placeholder.render(values, kind);
            result = result.replacen(&format!("${}$", placeholder.content), &rendered, 1);
        }
        result
    }

    pub fn value_default() -> Self {
        Template {
            content: "$__value__$".to_string(),
            placeholders: vec![Placeholder {
                name: "__value__".to_string(),
                content: "__value__".to_string(),
                prefix: None,
                suffix: None,
                delimiter: "".to_string(),
            }],
        }
    }

    pub fn children_default() -> Self {
        Template {
            content: "$__children__$".to_string(),
            placeholders: vec![Placeholder {
                name: "__children__".to_string(),
                content: "__children__".to_string(),
                prefix: None,
                suffix: None,
                delimiter: "".to_string(),
            }],
        }
    }
}
