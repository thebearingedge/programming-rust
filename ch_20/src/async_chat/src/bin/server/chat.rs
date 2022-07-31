use crate::connection::Client;
use async_chat_lib::FromServer;
use async_std::{sync::Arc, task};
use std::{collections::HashMap, sync::Mutex};
use tokio::sync::broadcast::{self, error::RecvError};

pub struct Chat(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl Chat {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn join_group(&self, group_name: Arc<String>, client: Arc<Client>) {
        self.0
            .lock()
            .unwrap()
            .entry(group_name.clone())
            .or_insert_with(|| Arc::new(Group::new(group_name)))
            .join(client);
    }

    pub fn post_to_group(&self, group_name: &String, message: Arc<String>) -> Option<()> {
        self.0
            .lock()
            .unwrap()
            .get(group_name)
            .map(|group| group.post(message))
    }
}

struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { name, sender }
    }

    fn join(&self, client: Arc<Client>) {
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

    fn post(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }
}
