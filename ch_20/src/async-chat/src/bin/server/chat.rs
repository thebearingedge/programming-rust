use async_std::prelude::*;
use async_std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    task,
};
use chat_lib::{
    utils::{self, ChatResult},
    FromServer,
};
use std::collections::HashMap;
use tokio::sync::broadcast::{self, error::RecvError};

pub struct Chat {
    groups: std::sync::Mutex<HashMap<Arc<String>, Arc<Group>>>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            groups: std::sync::Mutex::new(HashMap::new()),
        }
    }

    pub fn get_group(&self, group_name: &String) -> Option<Arc<Group>> {
        self.groups.lock().unwrap().get(group_name).cloned()
    }

    pub fn get_or_create_group(&self, group_name: Arc<String>) -> Arc<Group> {
        self.groups
            .lock()
            .unwrap()
            .entry(group_name.clone())
            .or_insert_with(|| Arc::new(Group::new(group_name)))
            .clone()
    }
}

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { name, sender }
    }

    pub fn join(&self, client: Arc<Client>) {
        let group_name = self.name.clone();
        let mut receiver = self.sender.subscribe();
        task::spawn(async move {
            loop {
                let message = match receiver.recv().await {
                    Ok(message) => FromServer::Message {
                        group_name: group_name.clone(),
                        message: message.clone(),
                    },
                    Err(RecvError::Lagged(n)) => {
                        FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
                    }
                    Err(RecvError::Closed) => break,
                };

                if client.send(message).await.is_err() {
                    break;
                }
            }
        });
    }

    pub fn post(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }
}

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
