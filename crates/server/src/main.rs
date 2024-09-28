use std::path::PathBuf;

use clap::{command, Parser};
use repositorystorage::RepositoryStorage;

mod connection;
mod repositorystorage;
mod server;

const HOST: &str = "127.0.0.1";
const PORT: usize = 6969;
const STORAGE_DIR: &str = "repositories";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Arguments {
    #[arg(short, long, short = 'a', help = "Server address to bind to")]
    address: Option<String>,
    #[arg(short, long, short = 'p', help = "Server port to bind to")]
    port: Option<usize>,
    #[arg(short, long, short = 'd', help = "Directory to store repositories")]
    dir: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    let host = args.address.unwrap_or(HOST.to_string());
    let port = args.port.unwrap_or(PORT);
    let storage_dir = PathBuf::from(args.dir.unwrap_or(STORAGE_DIR.to_string()));

    println!("Starting Server...");
    println!("Host: {}", host);
    println!("Port: {}", port);
    println!("Storage Directory: {:?}", storage_dir);

    let repository_storage = RepositoryStorage::new(storage_dir);
    repository_storage.init();

    let server = server::TitServer::new(host, port, repository_storage);
    server.run(); 
}