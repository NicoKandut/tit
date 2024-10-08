use crate::hashtree::HashTree;
use crate::terminal::CheckList;
use crate::util::{BinaryFileRead, BinaryFileWrite, TomlFileRead, TomlFileWrite};
use crate::{build_hash_tree_for_dir, util, InitError, Node, DOT_TIT};
use crate::{Commit, RepositoryState};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct TitRepository {
    root: PathBuf,
}

impl Default for TitRepository {
    fn default() -> Self {
        match util::find_tit_root() {
            Some(root) => Self::new(root),
            None => panic!("Directory is not part of a tit repository."),
        }
    }
}

impl TitRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn init(&self, name: &str, server: &str, branch: &str) -> Result<(), InitError> {
        let mut checklist = CheckList::new(&format!(
            "Initializing project '{}' in '{}'",
            name,
            self.root.display()
        ));

        let dot_tit_dir = self.root.join(DOT_TIT);
        if dot_tit_dir.exists() {
            return Err(InitError::AlreadyInitialized);
        }

        let root = Path::new(&self.root);

        // create directories
        checklist.start_step("Creating directories".to_string());

        fs::create_dir(&dot_tit_dir).map_err(|_| InitError::DirectoryCreateError(dot_tit_dir))?;
        let commits_dir = root.join(self.commits_dir());
        fs::create_dir(&commits_dir).map_err(|_| InitError::DirectoryCreateError(commits_dir))?;
        checklist.finish_step();

        // create state file
        checklist.start_step("Creating state file".to_string());
        let state_path = self.state_file();
        let state = RepositoryState::new(name.to_string(), branch.to_string(), server.to_string());
        state.write_to(&state_path);
        checklist.finish_step();

        // create tree file
        checklist.start_step("Creating hash-tree file".to_string());
        let tree_path = self.tree_file();
        let tree = HashTree::<Node>::default();
        tree.write_to(tree_path);
        checklist.finish_step();

        Ok(())
    }

    pub fn uninit(&self) -> Result<(), InitError> {
        let tit_dir = self.root.join(DOT_TIT);
        match fs::remove_dir_all(tit_dir) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::NotInitialized),
        }
    }

    fn commits_dir(&self) -> PathBuf {
        self.root.join(crate::DOT_TIT).join(crate::COMMIT_DIR)
    }

    fn commit_file(&self, commit_id: &str) -> PathBuf {
        self.commits_dir().join(commit_id)
    }

    fn state_file(&self) -> PathBuf {
        self.root.join(crate::DOT_TIT).join("state.toml")
    }

    fn tree_file(&self) -> PathBuf {
        self.root.join(crate::DOT_TIT).join("tree.bin")
    }

    pub fn write_commit(&self, commit: &Commit) {
        let commit_path = self.commit_file(&commit.get_id());
        commit.write_to(&commit_path);
    }

    pub fn read_commit(&self, id: &str) -> Commit {
        let commit_path = self.commit_file(id);
        Commit::read_from(&commit_path)
    }

    pub fn commit_ids(&self) -> Vec<String> {
        let commit_dir = self.commits_dir();
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
        RepositoryState::read_from(&self.state_file())
    }

    pub fn set_state(&self, state: RepositoryState) {
        state.write_to(&self.state_file())
    }

    pub fn signed_tree(&self) -> HashTree<Node> {
        HashTree::<_>::read_from(&self.tree_file())
    }

    pub fn current_tree(&self) -> HashTree<Node> {
        build_hash_tree_for_dir(self.root.as_path())
    }

    pub fn set_signed_tree(&self, after: HashTree<Node>) {
        after.write_to(self.tree_file());
    }
}
