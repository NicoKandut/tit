use network;
use std::net::TcpStream;

pub fn add_remote(server: &str, project: &str) {
    println!("Adding remote server: {} with project: {}", server, project);

    let mut stream = TcpStream::connect(server).expect("Failed to connect to server");
    network::write_message(&mut stream, network::TitClientMessage::Hello);

    match network::read_message(&mut stream) {
        network::TitServerMessage::Hello => println!("Server responded with Hello"),
        _ => println!("Server responded with Error"),
    }

    network::write_message(&mut stream, network::TitClientMessage::Disconnect);

    // add remote
}
