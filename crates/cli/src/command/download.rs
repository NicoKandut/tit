use network::TitServerMessage;

pub fn download() {


    println!("Contacting server.");
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository = kern::TitRepository::new(working_dir);
    let state = repository.state();

    let mut stream = std::net::TcpStream::connect(state.current_server).expect("Failed to connect to server");

    println!("Downloading index.");
    network::write_message(
        &mut stream,
        network::TitClientMessage::DownloadIndex {
            project: state.project_name,
        },
    );

    let message = network::read_message::<TitServerMessage>(&mut stream);
    let commits = match message {
        TitServerMessage::Index { commits } => commits,
        _ => panic!("Server responded with unexpected message"),
    };

    println!("Index contains {} commits", commits.len());

    for commit in commits {
        // check if the commit exists locally

        println!("  - Downloading commit {}", commit);
        network::write_message(
            &mut stream,
            network::TitClientMessage::DownloadFile { commit },
        );
        let message = network::read_message::<TitServerMessage>(&mut stream);
        let commit = match message {
            TitServerMessage::CommitFile { commit } => commit.clone(),
            _ => panic!("Server responded with unexpected message"),
        };

        repository.write_commit(&commit);
    }

    network::write_message(&mut stream, network::TitClientMessage::Disconnect);

    println!("Done");
}
