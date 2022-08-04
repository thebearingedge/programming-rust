use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::{
    sync::{
        broadcast::{self, error::RecvError},
        Mutex,
    },
    task,
};
use tokio_chat_lib::{
    utils::{self},
    FromServer,
};

pub struct Chat {
    groups: Mutex<HashMap<Arc<String>, Group>>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            groups: Mutex::new(HashMap::new()),
        }
    }

    pub async fn join(&self, group_name: Arc<String>, client: Arc<Mutex<OwnedWriteHalf>>) {
        self.groups
            .lock()
            .await
            .entry(group_name.clone())
            .or_insert_with(|| Group::new(group_name))
            .add_client(client);
    }

    pub async fn post_to(&self, group_name: &String, message: Arc<String>) -> Option<()> {
        self.groups
            .lock()
            .await
            .get(group_name)
            .map(|group| group.post(message))
    }
}

struct Group {
    name: Arc<String>,
    channel: broadcast::Sender<Arc<String>>,
}

impl Group {
    fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            name,
            channel: sender,
        }
    }

    fn post(&self, message: Arc<String>) {
        let _ = self.channel.send(message);
    }

    fn add_client(&self, client: Arc<Mutex<OwnedWriteHalf>>) {
        let group_name = self.name.clone();
        let mut group_messages = self.channel.subscribe();
        task::spawn(async move {
            loop {
                let payload = match group_messages.recv().await {
                    Ok(message) => FromServer::Message {
                        group_name: group_name.clone(),
                        message: message.clone(),
                    },
                    Err(RecvError::Lagged(n)) => {
                        FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
                    }
                    Err(RecvError::Closed) => break,
                };
                if utils::write_json(&mut *client.lock().await, &payload)
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });
    }
}
