use crate::repositorystorage::RepositoryStorage;
use network::TitServerMessage;
use std::net::TcpStream;

pub fn handle(mut stream: TcpStream, storage: RepositoryStorage) {
    let name = match network::read_message::<_>(&mut stream) {
        network::TitClientMessage::UseRepository { name } => name,
        _ => {
            println!("Received unexpected message");
            network::write_message(&mut stream, network::TitServerMessage::Error);
            return;
        }
    };

    println!("Client connected to repository: {}", name);
    let (repository, response) = storage.get_repository(&name).map_or_else(
        || {
            (
                storage.create_repository(&name).unwrap(),
                network::TitServerMessage::RepositoryCreated,
            )
        },
        |r| (r, network::TitServerMessage::Ok),
    );
    network::write_message(&mut stream, response);

    loop {
        match network::read_message::<_>(&mut stream) {
            network::TitClientMessage::Disconnect => {
                println!("Received Disconnect message");
                break;
            }
            network::TitClientMessage::DownloadIndex => {
                println!("Received DownloadIndex message");
                let commits = repository.commit_ids();
                let branches = repository.state().branches;
                let response = network::TitServerMessage::Index { commits, branches };
                network::write_message(&mut stream, response);
            }
            network::TitClientMessage::DownloadFile { id } => {
                println!("Received DownloadFile message: {}", id);
                let commit = kern::Commit::new("commit1".to_string(), vec![], 0, None);
                let response = network::TitServerMessage::CommitFile { commit };
                network::write_message(&mut stream, response);
            }
            network::TitClientMessage::UploadChanges { changes } => {
                println!("Received UploadFile message: {}", changes);
                repository.write_commit(&changes);
                network::write_message(&mut stream, network::TitServerMessage::Hello);
            }
            network::TitClientMessage::CreateRepository { name } => {
                let response = match storage.create_repository(&name) {
                    Ok(_) => TitServerMessage::RepositoryCreated,
                    Err(_) => TitServerMessage::Error,
                };
                network::write_message(&mut stream, response);
            }
            network::TitClientMessage::OfferContent { commits, branches } => {
                let missing_commits = set_difference(&commits, &repository.commit_ids());
                let response = network::TitServerMessage::RequestUpload {
                    commits: missing_commits,
                };
                network::write_message(&mut stream, response);

                let mut state = repository.state();
                for (name, commit_id) in branches {
                    state.branches.insert(name, commit_id);
                }
                repository.set_state(state);
            }
            _ => {
                println!("Received unexpected message");
                network::write_message(&mut stream, network::TitServerMessage::Error);
                break;
            }
        }
    }
}

pub fn set_difference<T: PartialEq>(a: &[T], b: &[T]) -> Vec<T> {
    let mut difference = vec![];

    for item in a {
        if !b.contains(item) {
            difference.push(item.clone());
        }
    }

    difference
}
