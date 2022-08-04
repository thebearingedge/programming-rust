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
            FromClient::Join { group_name } => {
                let group_member = to_client.clone();
                chat.join(group_name, group_member).await
            }
            FromClient::Post {
                group_name,
                message,
            } => match chat.post_to(&group_name, message).await {
                Some(_) => (),
                None => {
                    let message = format!("Group '{}' does not exist", group_name);
                    let err = FromServer::Error(message);
                    let mut write = to_client.lock().await;
                    utils::write_json(&mut *write, &err).await?;
                }
            },
        }
    }
    Ok(())
}
