use crate::{read_message, write_message, NetworkError, TitClientMessage, TitServerMessage};
use kern::Commit;
use std::{collections::BTreeMap, net::TcpStream};

#[derive(Debug)]
pub struct TitClient {
    pub stream: TcpStream,
}

impl Drop for TitClient {
    fn drop(&mut self) {
        let _ = write_message(&mut self.stream, TitClientMessage::Disconnect);
    }
}

impl TitClient {
    pub fn new(server: &str, project: &str) -> Result<Self, NetworkError> {
        let mut stream = TcpStream::connect(server).expect("Failed to connect to server");

        write_message(
            &mut stream,
            TitClientMessage::UseRepository {
                name: project.to_string(),
            },
        )?;

        match read_message::<TitServerMessage>(&mut stream) {
            Ok(TitServerMessage::RepositoryCreated | TitServerMessage::Ok) => Ok(Self { stream }),
            Err(e) => Err(e),
            _ => Err(NetworkError::UnexpectedMessage),
        }
    }

    pub fn download_index(
        &mut self,
    ) -> Result<(Vec<String>, BTreeMap<String, String>), NetworkError> {
        write_message(&mut self.stream, TitClientMessage::DownloadIndex)?;

        let message = read_message::<TitServerMessage>(&mut self.stream)?;
        match message {
            TitServerMessage::Index { commits, branches } => Ok((commits, branches)),
            _ => Err(NetworkError::UnexpectedMessage),
        }
    }

    pub fn download_commit(&mut self, id: String) -> Result<Commit, NetworkError> {
        write_message(&mut self.stream, TitClientMessage::DownloadFile { id })?;
        let message = read_message::<TitServerMessage>(&mut self.stream)?;
        match message {
            TitServerMessage::CommitFile { commit } => Ok(commit.clone()),
            _ => Err(NetworkError::UnexpectedMessage),
        }
    }

    pub fn offer_content(
        &mut self,
        commits: Vec<String>,
        branches: BTreeMap<String, String>,
    ) -> Result<Vec<String>, NetworkError> {
        write_message(
            &mut self.stream,
            TitClientMessage::OfferContent { commits, branches },
        )?;

        let message = read_message::<TitServerMessage>(&mut self.stream)?;
        match message {
            TitServerMessage::RequestUpload { commits } => Ok(commits),
            _ => Err(NetworkError::UnexpectedMessage),
        }
    }

    pub fn upload_changes(&mut self, changes: Vec<Commit>) -> Result<(), NetworkError> {
        let change_count = changes.len();
        for change in changes {
            write_message(
                &mut self.stream,
                TitClientMessage::UploadChanges { changes: change },
            )?;
        }
        for _ in 0..change_count {
            read_message::<TitServerMessage>(&mut self.stream)?;
        }

        Ok(())
    }
}
