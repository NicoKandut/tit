use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::error::RepositoryError;
use crate::{Branch, TIT_DIR};
use crate::{Commit, RepositoryState};

#[derive(Debug, Clone)]
pub struct TitRepository {
    root: PathBuf,
}

impl TitRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn init(&self) -> Result<(), RepositoryError> {
        let tit_exists = fs::read_dir(&self.root)
            .expect("Failed to read entries of cwd")
            .any(|entry| entry.expect("Failed to read entry").file_name() == crate::TIT_DIR);

        if tit_exists {
            return Err(RepositoryError("Repository already initialized!", None));
        }

        let root = Path::new(&self.root);
        println!("Initializing project...");

        // create folders
        fs::create_dir(root.join(TIT_DIR))
            .map_err(|e| RepositoryError("Failed to create .tit folder", Some(e)))?;
        fs::create_dir(root.join(self.get_commits_dir()))
            .map_err(|e| RepositoryError("Failed to create commits folder", Some(e)))?;
        fs::create_dir(root.join(self.get_branch_dir()))
            .map_err(|e| RepositoryError("Failed to create branch folder", Some(e)))?;

        // create state file
        let state_path = self.get_state_file();
        let state = RepositoryState::new(
            "project".to_string(),
            "main".to_string(),
            "none".to_string(),
            "none".to_string(),
        );
        RepositoryState::save(&state_path, state);
        println!("Done");
        Ok(())
    }

    pub fn uninit(&self) {
        let tit_dir = Path::new(&self.root).join(TIT_DIR);
        fs::remove_dir_all(tit_dir).expect("Failed to uninit repository.")
    }

    //<editor-fold> File & Directory Paths

    fn get_commits_dir(&self) -> PathBuf {
        self.root.join(crate::TIT_DIR).join(crate::COMMIT_DIR)
    }

    fn get_commit_file(&self, commit_id: &str) -> PathBuf {
        self.get_commits_dir().join(commit_id)
    }

    fn get_branch_dir(&self) -> PathBuf {
        self.root.join(crate::TIT_DIR).join(crate::BRANCH_DIR)
    }

    fn get_branch_file(&self, branch_id: &str) -> PathBuf {
        self.get_branch_dir().join(branch_id)
    }

    fn get_state_file(&self) -> PathBuf {
        self.root.join(crate::TIT_DIR).join("state.toml")
    }

    //</editor-fold>

    pub fn write_commit(&self, commit: &Commit) {
        let config = bincode::config::standard();
        let commit_path = self.get_commit_file(&commit.get_id());
        let bytes =
            bincode::encode_to_vec::<_, _>(commit, config).expect("Failed to encode commit!");

        fs::File::create(commit_path.clone())
            .expect(&format!(
                "Failed to create commit file! Path: {commit_path:?}"
            ))
            .write(&bytes)
            .expect("Failed to write commit!");
    }

    pub fn read_commit(&self, id: &str) -> Commit {
        let config = bincode::config::standard();
        let commit_path = self.get_commit_file(id);
        let mut bytes = vec![];
        fs::File::open(commit_path)
            .expect("Failed to open commit file!")
            .read_to_end(&mut bytes)
            .expect("Failed to read commit file!");
        let (commit, _) =
            bincode::decode_from_slice(&bytes, config).expect("Failed to decode commit");
        commit
    }

    pub fn switch_branch(&self, branch_id: &str) {
        let branch_path = self.get_branch_file(branch_id);

        let exists = fs::metadata(&branch_path).is_ok();

        if !exists {
            panic!("Branch does not exist!");
        }
    }

    pub fn commit_ids(&self) -> Vec<String> {
        let commit_dir = self.get_commits_dir();
        fs::read_dir(commit_dir)
            .expect("Failed to read commits directory!")
            .map(|entry| {
                entry
                    .expect("Failed to read entry.")
                    .file_name()
                    .into_string()
                    .unwrap()
            })
            .collect::<_>()
    }

    pub fn commits(&self) -> HashMap<String, Commit> {
        self.commit_ids()
            .into_iter()
            .map(|id| (id.clone(), self.read_commit(&id)))
            .collect::<_>()
    }

    pub fn state(&self) -> RepositoryState {
        RepositoryState::load(&self.get_state_file())
    }

    pub fn set_state(&self, state: RepositoryState) {
        RepositoryState::save(&self.get_state_file(), state)
    }

    pub fn write_branch(&self, branch: &Branch) {
        let file_path = self.get_branch_file(&branch.name);
        fs::File::create(file_path.clone())
            .expect(&format!(
                "Failed to create branch file! Path: {file_path:?}"
            ))
            .write_all(branch.commit_id.as_bytes())
            .expect("Failed to write branch!");
    }

    pub fn read_branch(&self, branch_name: &str) -> Branch {
        let file_path = self.get_branch_file(branch_name);
        let mut commit_id = String::new();
        fs::File::open(file_path)
            .expect("Failed to open branch file!")
            .read_to_string(&mut commit_id)
            .expect("Failed to read branch file!");
        Branch::new(branch_name.to_string(), commit_id)
    }
}
