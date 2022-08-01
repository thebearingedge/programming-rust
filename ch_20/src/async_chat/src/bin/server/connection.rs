use crate::chat::Chat;
use async_chat_lib::{
    utils::{self, ChatResult},
    FromClient, FromServer,
};
use async_std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
    prelude::*,
    sync::{Arc, Mutex},
};

pub struct Client {
    input: TcpStream,
    output: Mutex<TcpStream>,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            input: socket.clone(),
            output: Mutex::new(socket),
        }
    }

    pub async fn send(&self, message: FromServer) -> ChatResult<()> {
        let mut socket = self.output.lock().await;
        utils::write_json(&mut *socket, &message).await?;
        socket.flush().await?;
        Ok(())
    }

    fn get_input_stream(&self) -> impl Stream<Item = ChatResult<FromClient>> {
        utils::read_json(BufReader::new(self.input.clone()))
    }
}

pub async fn serve(client: Arc<Client>, chat: Arc<Chat>) -> ChatResult<()> {
    let mut from_client = client.get_input_stream();
    while let Some(payload) = from_client.next().await {
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
