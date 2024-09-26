use network::{TitClientMessage, TitServerMessage};

pub fn create_repo(server: &str, name: &str) {
    println!("Contacting server.");
    let mut stream = std::net::TcpStream::connect(server).expect("Failed to connect to server");

    println!("Creating repository.");
    network::write_message(
        &mut stream,
        network::TitClientMessage::CreateRepository {
            name: name.to_string(),
        },
    );

    let message = network::read_message::<TitServerMessage>(&mut stream);
    match message {
        TitServerMessage::RepositoryCreated => {}
        _ => panic!("Server responded with unexpected message"),
    };

    network::write_message(&mut stream, TitClientMessage::Disconnect);

    println!("Done");
}

pub fn upload_all(server: &str, project: &str) {
    println!("Syncing commits");
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let mut stream = std::net::TcpStream::connect(server).expect("Failed to connect to server");
    let local_commits = repository.get_commits();

    network::write_message(
        &mut stream,
        network::TitClientMessage::OfferCommits {
            project: project.to_string(),
            commits: local_commits,
        },
    );

    let message = network::read_message::<TitServerMessage>(&mut stream);
    let commits_to_upload = match message {
        TitServerMessage::RequestCommitUpload { commits } => commits,
        _ => vec![],
    };

    println!(
        "Server needs {} commits to be uploaded.",
        commits_to_upload.len()
    );

    for commit_id in commits_to_upload.iter() {
        let commit = repository.read_commit(&commit_id);
        println!("  - Uploading commit: {}", commit_id);
        network::write_message(
            &mut stream,
            TitClientMessage::UploadFile {
                commit,
                project: project.to_string(),
            },
        );
    }

    for _ in commits_to_upload.iter() {
        network::read_message::<TitServerMessage>(&mut stream);
    }

    network::write_message(&mut stream, TitClientMessage::Disconnect);

    println!("Done");
}
