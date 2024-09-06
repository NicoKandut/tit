use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::commit::Commit;

#[derive(Debug, Clone)]
pub struct TitRepository {
    root: PathBuf,
}

impl TitRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn try_init(&self) {
        let tit_exists = fs::read_dir(&self.root)
            .expect("Failed to read entries of cwd")
            .any(|entry| entry.expect("Failed to read entry").file_name() == crate::TIT_DIR);

        if tit_exists {
            println!("Repository already initialized!");
            return;
        }

        self.init()
    }

    fn init(&self) {
        println!("Initializing project...");
        fs::create_dir(crate::TIT_DIR).expect("Failed to create .tit folder");
        println!("Done");
    }

    //<editor-fold> File & Directory Paths

    fn get_commits_dir(&self) -> PathBuf {
        self.root.join(crate::TIT_DIR).join(crate::COMMIT_DIR)
    }

    fn get_commit_file(&self, commit_id: &str) -> PathBuf {
        self.get_commits_dir().join(commit_id)
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
}
