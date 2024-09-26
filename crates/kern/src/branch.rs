
#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub commit_id: String,
}

impl Branch {
    pub fn new(name: String, commit_id: String) -> Self {
        Self { name, commit_id }
    }
}
