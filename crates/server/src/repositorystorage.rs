use std::{
    fmt::format,
    fs,
    path::{self, Path, PathBuf},
};

use kern::TitRepository;

const REPOSITORIES_DIR: &str = "repos";

#[derive(Clone, Debug)]
pub struct RepositoryStorage {
    root_dir: String,
}

impl RepositoryStorage {
    pub fn new(root_dir: &str) -> Self {
        Self {
            root_dir: root_dir.to_string(),
        }
    }

    pub fn init(&self) {
        let exists = fs::read_dir(&self.root_dir)
            .expect("Failed to read entries of cwd")
            .any(|entry| entry.expect("Failed to read entry").file_name() == REPOSITORIES_DIR);

        if !exists {
            fs::create_dir(REPOSITORIES_DIR).expect("Failed to create repositories directory.");
        }
    }

    pub fn create_repository(&self, name: &str) -> Result<(), &'static str> {
        if self.repository_exists(name) {
            return Err("Repository already exists.");
        }

        let path = self.get_repo_path(name);

        if fs::create_dir(path.clone()).is_err() {
            return Err("Failed to create repository.");
        }

        let repository = TitRepository::new(path.clone());
        if repository.init().is_err() {
            return Err("Failed to initialize repository.");
        }

        println!("Initialized repository");

        Ok(())
    }

    pub fn get_repository(&self, name: &str) -> Option<TitRepository> {
        if !self.repository_exists(name) {
            None
        } else {
            let repo_path = self.get_repo_path(name);
            Some(TitRepository::new(repo_path))
        }
    }

    fn get_repo_path(&self, name: &str) -> PathBuf {
        Path::new(&self.root_dir)
        .join(REPOSITORIES_DIR)
        .join(name)
    }

    fn repository_exists(&self, name: &str) -> bool {
        let path = Path::new(&self.root_dir).join(REPOSITORIES_DIR);
        fs::read_dir(path.clone())
            .expect("Failed to read entries of cwd")
            .any(|entry| entry.expect("Failed to read entry").file_name() == name)
    }
}
