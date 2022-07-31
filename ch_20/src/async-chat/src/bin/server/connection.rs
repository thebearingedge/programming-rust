use async_std::prelude::*;
use async_std::{
    io::BufReader,
    net::TcpStream,
    sync::{Arc, Mutex},
};
use chat_lib::utils::{self, ChatResult};
use chat_lib::{FromClient, FromServer};

use crate::group_table::GroupTable;

pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    let outbound = Arc::new(Outbound::new(socket.clone()));
    let mut stream = utils::read_json(BufReader::new(socket));
    while let Some(data) = stream.next().await {
        match data? {
            FromClient::Join { group_name } => {
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());
            }
            FromClient::Post {
                group_name,
                message,
            } => match groups.get(&group_name) {
                Some(group) => group.post(message),
                None => {
                    let err = FromServer::Error(format!("Group '{}' does not exist", group_name));
                    outbound.send(err).await?;
                }
            },
        };
    }
    Ok(())
}

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
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
