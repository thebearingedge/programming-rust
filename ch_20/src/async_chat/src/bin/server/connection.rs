use crate::chat::Chat;
use async_chat_lib::{
    utils::{self, ChatResult},
    FromClient, FromServer,
};
use async_std::{
    io::BufReader,
    net::TcpStream,
    prelude::*,
    sync::{Arc, Mutex},
};

pub struct Client(Mutex<TcpStream>);

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Self(Mutex::new(socket))
    }

    pub async fn send(&self, message: FromServer) -> ChatResult<()> {
        let mut socket = self.0.lock().await;
        utils::write_json(&mut *socket, &message).await?;
        socket.flush().await?;
        Ok(())
    }
}

pub async fn serve(socket: TcpStream, chat: Arc<Chat>) -> ChatResult<()> {
    let client = Arc::new(Client::new(socket.clone()));
    let mut stream = utils::read_json(BufReader::new(socket));
    while let Some(payload) = stream.next().await {
        match payload? {
            FromClient::Join { group_name } => chat.join_group(group_name, client.clone()),
            FromClient::Post {
                group_name,
                message,
            } => match chat.post_to_group(&group_name, message) {
                None => {
                    let err = FromServer::Error(format!("Group '{}' does not exist", group_name));
                    client.send(err).await?;
                }
                _ => (),
            },
        };
    }
    Ok(())
}
