use kern::TitRepository;
use network::TitClient;

pub fn sync() {
    let repository = TitRepository::default();
    let state = repository.state();
    let server_name = state.current.server;
    let server_address = state.servers.get(&server_name).unwrap();
    let mut checklist = kern::terminal::CheckList::new("Syncing repository");

    checklist.start_step(format!(
        "Contacting server {} ({}).",
        server_name, server_address
    ));
    let mut client = TitClient::new(server_address, &state.project.name);
    checklist.finish_step();

    checklist.start_step(format!("Downloading index"));
    let (commits, branches) = match client.download_index() {
        Ok(index) => index,
        Err(e) => panic!("Failed to download index: {}", e),
    };
    checklist.finish_step();

    checklist.start_step("Downloading commits".to_string());
    let commits = commits
        .into_iter()
        .filter(|commit| !repository.commit_ids().contains(commit))
        .collect::<Vec<_>>();
    for id in commits {
        match client.download_commit(id) {
            Ok(commit) => repository.write_commit(&commit),
            Err(e) => panic!("Failed to download commit: {}", e),
        }
    }
    checklist.finish_step();

    checklist.start_step("Offering changes to server".to_string());
    let local_commits = repository.commit_ids();
    let missing_commit_ids = client.offer_content(local_commits, state.branches.clone());

    checklist.finish_step();

    checklist.start_step(format!(
        "Uploading changes: {} changes",
        missing_commit_ids.len()
    ));
    let commits_to_upload = missing_commit_ids
        .iter()
        .map(|id| repository.read_commit(id))
        .collect();
    client.upload_changes(commits_to_upload);
    checklist.finish_step();

    checklist.start_step("Updating branches".to_string());
    let mut state = repository.state();
    for (branch, commit) in branches {
        if state.branches.get(&branch) != Some(&commit) {
            state.branches.insert(branch, commit);
        }
    }
    repository.set_state(state.clone());
    checklist.finish_step();

    println!("Done");
}
