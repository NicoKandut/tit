use repositorystorage::RepositoryStorage;

mod connection;
mod repositorystorage;
mod server;

const HOST: &str = "127.0.0.1";
const PORT: usize = 6969;

fn main() {
    println!("Starting Server..");

    // TODO: extract to cli parameter
    let working_dir = std::env::current_dir().expect("Failed to get current working directory!");
    let repository_storage = RepositoryStorage::new(working_dir.to_str().unwrap());
    repository_storage.init();

    let server = server::TitServer::new(HOST.to_string(), PORT, repository_storage);
    server.run(); 
}