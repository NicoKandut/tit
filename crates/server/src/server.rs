use crate::{connection, repositorystorage::RepositoryStorage};
use std::net::TcpListener;

pub struct TitServer {
    host: String,
    port: usize,
    storage: RepositoryStorage,
}

impl TitServer {
    pub fn new(host: String, port: usize, storage: RepositoryStorage) -> Self {
        Self {
            host,
            port,
            storage,
        }
    }

    pub fn run(&self) {
        let address: String = format!("{}:{}", self.host, self.port);
        let listener =
            TcpListener::bind(address).expect(&format!("Server failed to bind to {}", self.port));
        println!("Server listening on port 6969");
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("New connection: {}", addr);
                    let storage = self.storage.clone();
                    std::thread::spawn(move || connection::handle(stream, storage));
                }
                Err(e) => {
                    println!("Fatal Server Error: {}", e);
                }
            }
        }
    }
}
