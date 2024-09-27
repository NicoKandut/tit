use kern::TitRepository;
use network::TitClient;

pub fn sync() {
    let repository = TitRepository::default();
    let state = repository.state();
    let server_name = state.current.server;
    let server_address = state.servers.get(&server_name).unwrap();

    println!("Contacting server {} ({}).", server_name, server_address);
    let mut client = TitClient::new(server_address, &state.project.name);

    println!("Downloading index.");
    let (commits, branches) = match client.download_index() {
        Ok(index) => index,
        Err(e) => panic!("Failed to download index: {}", e),
    };

    let commits = commits
        .into_iter()
        .filter(|commit| !repository.commit_ids().contains(commit))
        .collect::<Vec<_>>();

    println!("Downloading commits...");
    for id in commits {
        println!("  - Downloading commit {}", id);
        match client.download_commit(id) {
            Ok(commit) => repository.write_commit(&commit),
            Err(e) => panic!("Failed to download commit: {}", e),
        }
    }

    let mut state = repository.state();
    for (branch, commit) in branches {
        if state.branches.get(&branch) == Some(&commit) {
            continue;
        }

        println!("  - Updating branch {} to commit {}", branch, commit);
        state.branches.insert(branch, commit);
    }

    let local_commits = repository.commit_ids();
    println!("Offering {} commits to server.", local_commits.len());
    let missing_commit_ids = client.offer_content(local_commits, state.branches.clone());

    println!("Uploading changes: {} changes", missing_commit_ids.len());

    let commits_to_upload = missing_commit_ids
        .iter()
        .map(|id| repository.read_commit(id))
        .collect();
    client.upload_changes(commits_to_upload);
    repository.set_state(state);

    println!("Done");
}
