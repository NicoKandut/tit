use network::{TitClientMessage, TitServerMessage};

pub fn create_repo(name: &str) {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let state = repository.state();

    println!("Contacting server.");
    let mut stream =
        std::net::TcpStream::connect(state.current.server).expect("Failed to connect to server");

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

pub fn upload_all() {
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let state = repository.state();

    println!("Contacting server {}.", state.current.server);
    let mut stream =
        std::net::TcpStream::connect(state.current.server).expect("Failed to connect to server");

    println!("Syncing Changes...");
    let local_commits = repository.commit_ids();
    println!("Offering {} commits to server.", local_commits.len());
    network::write_message(
        &mut stream,
        network::TitClientMessage::OfferCommits {
            project: state.project.name.clone(),
            commits: local_commits,
        },
    );

    let message = network::read_message::<TitServerMessage>(&mut stream);
    let commits_to_upload = match message {
        TitServerMessage::RequestCommitUpload { commits } => commits,
        _ => vec![],
    };

    println!(
        "Server needs {} changes to be uploaded.",
        commits_to_upload.len()
    );

    println!("Uploading changes: {} changes", commits_to_upload.len());
    commits_to_upload
        .iter()
        .map(|id| repository.read_commit(&id))
        .for_each(|changes| {
            network::write_message(
                &mut stream,
                TitClientMessage::UploadChanges {
                    changes,
                    project: state.project.name.clone(),
                },
            );
        });
    for _ in 0..commits_to_upload.len() {
        network::read_message::<TitServerMessage>(&mut stream);
    }

    network::write_message(&mut stream, TitClientMessage::Disconnect);

    println!("Done");
}
