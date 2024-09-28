use std::{
    fs,
    path::{Path, PathBuf},
};

use kern::TitRepository;

#[derive(Clone, Debug)]
pub struct RepositoryStorage {
    dir: PathBuf,
}

impl RepositoryStorage {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn init(&self) {
        if !self.dir.exists() {
            fs::create_dir(&self.dir).expect("Failed to create working directory.");
        }
    }

    pub fn create_repository(&self, name: &str) -> Result<TitRepository, &'static str> {
        if self.repository_exists(name) {
            return Err("Repository already exists.");
        }

        let path = self.get_repo_path(name);

        if fs::create_dir(path.clone()).is_err() {
            return Err("Failed to create repository.");
        }

        let repository = TitRepository::new(path.clone());

        repository
            .init(name, "self", "none")
            .expect("Failed to init repository.");

        println!("Created repository at {:?}", path);

        Ok(repository)
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
        Path::new(&self.dir).join(name)
    }

    fn repository_exists(&self, name: &str) -> bool {
        self.get_repo_path(name).exists()
    }
}
