use std::collections::HashMap;

pub type ChildDelimiters = HashMap<String, String>;

pub struct Template {
    #[allow(dead_code)]
    pub name: String,
    pub content: String,
    pub children_delimiters: ChildDelimiters,
}

pub const FALL_BACK_TEMPLATE_VALUE: &str = "$__value__$";
pub const FALL_BACK_TEMPLATE_CHILDREN: &str = "$__children__$";
pub const TEMPLATE_PATTERN: &str = r"\$([A-Za-z_])+\$";
pub const TEMPLATE_PATTERN_CHILDREN: &str = r"\$__children(?::([^_]+))?__\$";