use network;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").expect("Server failed to bind to port 6969");
    println!("Server listening on port 6969");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection: {}", addr);
                std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Fatal Server Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: std::net::TcpStream) {
    loop {
        let message = network::read_message::<_>(&mut stream);

        match message {
            network::TitClientMessage::Hello => {
                println!("Received Hello message");
                network::write_message(&mut stream, network::TitServerMessage::Hello);
            }
            network::TitClientMessage::Error => {
                println!("Received Error message");
                network::write_message(&mut stream, network::TitServerMessage::Error);
            }
            network::TitClientMessage::Disconnect => {
                println!("Received Disconnect message");
                break;
            }
        }
    }
}
