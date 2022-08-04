use crate::chat::Chat;
use std::sync::Arc;
use tokio::{io::BufReader, net::TcpStream, sync::Mutex};
use tokio_chat_lib::{
    utils::{self, ChatResult},
    FromClient, FromServer,
};
use tokio_stream::StreamExt;

pub async fn serve(chat: Arc<Chat>, socket: TcpStream) -> ChatResult<()> {
    let (inbound, outbound) = socket.into_split();
    let to_client = Arc::new(Mutex::new(outbound));
    let mut from_client = utils::read_json(BufReader::new(inbound));
    while let Some(payload) = from_client.next().await {
        match payload? {
            FromClient::Join { group_name } => chat.join(group_name, to_client.clone()).await,
            FromClient::Post {
                group_name,
                message,
            } => match chat.post_to(&group_name, message).await {
                None => {
                    let err = FromServer::Error(format!("Group '{}' does not exist", group_name));
                    let mut write = to_client.lock().await;
                    utils::write_json(&mut *write, &err).await?;
                }
                _ => (),
            },
        }
    }
    Ok(())
}
