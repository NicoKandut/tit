use std::path::Path;
use crate::errors::TemplateLoadingError;
use crate::palceholder::Placeholder;

pub struct Template<'a> {
    pub content: &'a str,
    pub placeholders: Vec<Placeholder<'a>>,
}

// pub fn load_template<'a>(folder: &'a Path, name: &'a str) -> Result<Template<'a>, TemplateLoadingError> {
//     let path = folder.join(name);
//     if !path.exists() {
//         return Err(TemplateLoadingError::NotFoundError);
//     }
//     
//     let content = std::fs::read_to_string(&path).map_err(TemplateLoadingError::IoError)?;
//     
// }