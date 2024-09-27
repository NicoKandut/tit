use std::{collections::BTreeMap, net::TcpStream};

use kern::Commit;

use crate::{read_message, write_message, TitClientMessage, TitServerMessage};

#[derive(Debug)]
pub struct TitClient {
    pub stream: TcpStream,
}

impl Drop for TitClient {
    fn drop(&mut self) {
        write_message(&mut self.stream, TitClientMessage::Disconnect);
    }
}

impl TitClient {
    pub fn new(server: &str, project: &str) -> Self {
        let mut stream = TcpStream::connect(server).expect("Failed to connect to server");

        write_message(
            &mut stream,
            TitClientMessage::UseRepository {
                name: project.to_string(),
            },
        );

        match read_message::<TitServerMessage>(&mut stream) {
            TitServerMessage::RepositoryCreated | TitServerMessage::Ok => {}
            _ => panic!("Server responded with unexpected message"),
        }

        Self { stream }
    }

    pub fn download_index(&mut self) -> Result<(Vec<String>, BTreeMap<String, String>), &str> {
        write_message(&mut self.stream, TitClientMessage::DownloadIndex {});

        let message = read_message::<TitServerMessage>(&mut self.stream);
        match message {
            TitServerMessage::Index { commits, branches } => Ok((commits, branches)),
            _ => Err("Server responded with unexpected message"),
        }
    }

    pub fn download_commit(&mut self, id: String) -> Result<Commit, &str> {
        write_message(&mut self.stream, TitClientMessage::DownloadFile { id });
        let message = read_message::<TitServerMessage>(&mut self.stream);
        match message {
            TitServerMessage::CommitFile { commit } => Ok(commit.clone()),
            _ => Err("Server responded with unexpected message"),
        }
    }

    pub fn offer_content(
        &mut self,
        commits: Vec<String>,
        branches: BTreeMap<String, String>,
    ) -> Vec<String> {
        write_message(
            &mut self.stream,
            TitClientMessage::OfferContent { commits, branches },
        );

        let message = read_message::<TitServerMessage>(&mut self.stream);
        match message {
            TitServerMessage::RequestUpload { commits } => commits,
            _ => vec![],
        }
    }

    pub fn upload_changes(&mut self, changes: Vec<Commit>) {
        let change_count = changes.len();
        for change in changes {
            write_message(
                &mut self.stream,
                TitClientMessage::UploadChanges { changes: change },
            );
        }
        for _ in 0..change_count {
            read_message::<TitServerMessage>(&mut self.stream);
        }
    }
}
