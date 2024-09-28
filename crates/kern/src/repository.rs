use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::error::RepositoryError;
use crate::{util, TIT_DIR};
use crate::{Commit, RepositoryState};

#[derive(Debug, Clone)]
pub struct TitRepository {
    root: PathBuf,
}

impl Default for TitRepository {
    fn default() -> Self {
        match util::find_tit_root() {
            Some(root) => Self { root },
            None => panic!("Directory is not part of a tit repository."),
        }
    }
}

impl TitRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn init(&self, name: &str, server: &str, branch: &str) -> Result<(), RepositoryError> {
        let tit_exists = fs::read_dir(&self.root)
            .expect("Failed to read entries of cwd")
            .any(|entry| entry.expect("Failed to read entry").file_name() == crate::TIT_DIR);

        if tit_exists {
            return Err(RepositoryError("Repository already initialized!", None));
        }

        let root = Path::new(&self.root);
        println!("Initializing project {name}");

        // create directories
        fs::create_dir(root.join(TIT_DIR))
            .map_err(|e| RepositoryError("Failed to create .tit folder", Some(e)))?;
        fs::create_dir(root.join(self.get_commits_dir()))
            .map_err(|e| RepositoryError("Failed to create commits folder", Some(e)))?;

        // create state file
        let state_path = self.get_state_file();
        let state = RepositoryState::new(name.to_string(), branch.to_string(), server.to_string());
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

    fn get_state_file(&self) -> PathBuf {
        self.root.join(crate::TIT_DIR).join("state.toml")
    }

    //</editor-fold>

    pub fn write_commit(&self, commit: &Commit) {
        let config = bincode::config::standard();
        let commit_path = self.get_commit_file(&commit.get_id());
        let bytes =
            bincode::encode_to_vec::<_, _>(commit, config).expect("Failed to encode commit!");
        let compressed_bytes = miniz_oxide::deflate::compress_to_vec(&bytes, 10);
        fs::File::create(commit_path.clone())
            .expect(&format!(
                "Failed to create commit file! Path: {commit_path:?}"
            ))
            .write(&compressed_bytes)
            .expect("Failed to write commit!");
    }

    pub fn read_commit(&self, id: &str) -> Commit {
        let config = bincode::config::standard();
        let commit_path = self.get_commit_file(id);
        let mut compressed_bytes = vec![];
        fs::File::open(commit_path)
            .expect("Failed to open commit file!")
            .read_to_end(&mut compressed_bytes)
            .expect("Failed to read commit file!");
        let bytes = miniz_oxide::inflate::decompress_to_vec(&compressed_bytes).expect("Failed to decompress commit!");
        let (commit, _) =
            bincode::decode_from_slice(&bytes, config).expect("Failed to decode commit");
        commit
    }

    pub fn switch_branch(&self, branch_id: &str) {
        let mut state = self.state();

        if (state.branches.get(branch_id)).is_none() {
            panic!("Branch does not exist!");
        }

        state.current.branch = branch_id.to_string();
        self.set_state(state);
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
}
