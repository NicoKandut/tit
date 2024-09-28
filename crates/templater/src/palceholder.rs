#[derive(Clone)]
pub struct Fix {
    pub key: String,
    pub value: String,
    pub is_negated: bool,
}

#[derive(Clone)]
pub struct Placeholder {
    pub name: String,
    pub content: String,
    pub prefix: Option<Fix>,
    pub suffix: Option<Fix>,
    pub delimiter: String,
}

impl Placeholder {
    pub fn from_caps(caps: &regex::Captures) -> Self {
        let content = caps.name("placeholder").expect("Placeholder should exist").as_str().to_string();
        let name = caps.name("role").expect("Role should exist").as_str().to_string();

        let mut prefix = None;
        if let Some(_) = caps.name("prefix") {
            let prefix_key = caps.name("prefix_key").expect("Prefix key should exist").as_str().to_string();
            let prefix_value = caps.name("prefix_value").expect("Prefix value should exist").as_str().to_string();
            let is_negated = caps.name("prefix_not").is_some();
            prefix = Some(Fix {
                key: prefix_key,
                value: prefix_value,
                is_negated,
            });
        }
        
        let mut suffix = None;
        if let Some(_) = caps.name("suffix") {
            let suffix_key = caps.name("suffix_key").expect("Suffix key should exist").as_str().to_string();
            let suffix_value = caps.name("suffix_value").expect("Suffix value should exist").as_str().to_string();
            let is_negated = caps.name("suffix_not").is_some();
            suffix = Some(Fix {
                key: suffix_key,
                value: suffix_value,
                is_negated,
            });
        }
        
        let mut delimiter = "".to_string();
        if let Some(_) = caps.name("delim") {
            delimiter = caps.name("delim_value").expect("Delimiter value should exist").as_str().to_string();
        }

        Self {
            name,
            content,
            prefix,
            suffix,
            delimiter,
        }
    }
    
    pub fn render(&self, values: &[String], kind: &str) -> String {
        let mut result = String::new();
        if let Some(prefix) = &self.prefix {
            if prefix.key == kind && !prefix.is_negated || prefix.key != kind && prefix.is_negated {
                result = prefix.value.clone();
            }
        }
        
        result.push_str(&values.join(&self.delimiter));
        
        if let Some(suffix) = &self.suffix {
            if suffix.key == kind && !suffix.is_negated || suffix.key != kind && suffix.is_negated {
                result.push_str(&suffix.value);
            }
        }
        
        result
    }
}