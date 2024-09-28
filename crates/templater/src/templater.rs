use std::collections::HashMap;
use std::path::Path;
use crate::template::Template;

pub struct Templater<'a> {
    folder: &'a Path,
    templates: HashMap<String, Template<'a>>
}