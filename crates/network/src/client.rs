pub struct TitClient {}
 
impl TitClient {
    pub fn create_repo(name: &str) -> Result<(), &str> {
        let mut stream = std::net::TcpStream::connect(server).expect("Failed to connect to server");
        network::write_message(
            &mut stream,
            network::TitClientMessage::CreateRepository {
                name: name.to_string(),
            },
        );

        let message = network::read_message::<TitServerMessage>(&mut stream);
        match message {
            TitServerMessage::RepositoryCreated => {}
            _ => return Err("Received unexpected message."),
        };

        network::write_message(&mut stream, TitClientMessage::Disconnect);

        Ok(())
    }
}
