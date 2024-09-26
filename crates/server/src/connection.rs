use std::net::TcpStream;

use network::TitServerMessage;

use crate::repositorystorage::RepositoryStorage;

pub fn handle(mut stream: TcpStream, storage: RepositoryStorage) {
    loop {
        match network::read_message::<_>(&mut stream) {
            network::TitClientMessage::Hello => handle_hello(&mut stream),
            network::TitClientMessage::Error => {
                println!("Received Error message");
                network::write_message(&mut stream, network::TitServerMessage::Error);
            }
            network::TitClientMessage::DownloadIndex { project } => {
                println!("Received DownloadIndex message");
                let response = match storage.get_repository(&project) {
                    Some(repo) => {
                        let commits = repo.commit_ids();
                        network::TitServerMessage::Index { commits }
                    }
                    None => TitServerMessage::Error,
                };
                network::write_message(&mut stream, response);
            }
            network::TitClientMessage::DownloadFile { commit } => {
                println!("Received DownloadFile message: {}", commit);
                let commit_data = kern::Commit::new("commit1".to_string(), vec![], 0, None);
                network::write_message(
                    &mut stream,
                    network::TitServerMessage::CommitFile {
                        commit: commit_data,
                    },
                );
            }
            network::TitClientMessage::UploadFile { project, changes } => {
                println!("Received UploadFile message: {}", changes);

                if let Some(repo) = storage.get_repository(&project) {
                    repo.write_commit(&changes);
                }

                network::write_message(&mut stream, network::TitServerMessage::Hello);
            }
            network::TitClientMessage::Disconnect => {
                println!("Received Disconnect message");
                break;
            }
            network::TitClientMessage::CreateRepository { name } => {
                let response = match storage.create_repository(&name) {
                    Ok(()) => TitServerMessage::RepositoryCreated,
                    Err(e) => TitServerMessage::Error,
                };
                network::write_message(&mut stream, response);
            }
            network::TitClientMessage::OfferCommits { commits, project } => {
                let missing_commits = match storage.get_repository(&project) {
                    Some(repo) => set_difference(&commits, &repo.commit_ids()),
                    None => vec![],
                };
                network::write_message(
                    &mut stream,
                    network::TitServerMessage::RequestCommitUpload {
                        commits: missing_commits,
                    },
                );
            }
        }
    }
}

fn handle_hello(stream: &mut TcpStream) {
    println!("Received Hello message");
    network::write_message(stream, network::TitServerMessage::Hello);
}



pub fn set_difference(a: &[String], b: &[String]) -> Vec<String> {
    let mut difference = vec![];

    for item in a {
        if !b.contains(item) {
            difference.push(item.clone());
        }
    }

    difference
}
