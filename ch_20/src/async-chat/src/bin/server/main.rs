use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    prelude::*,
    sync::Arc,
    task,
};
use chat_lib::{
    utils::{self, ChatError, ChatResult},
    FromClient, FromServer,
};

mod chat;

#[async_std::main]
async fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");
    let chat = Arc::new(chat::Chat::new());
    task::block_on(async {
        let listener = TcpListener::bind(address).await?;
        while let Some(socket_result) = listener.incoming().next().await {
            let socket = socket_result?;
            let chat = chat.clone();
            task::spawn(async move {
                let client = Arc::new(chat::Client::new(socket.clone()));
                let mut stream =
                    utils::read_json::<BufReader<TcpStream>, FromClient>(BufReader::new(socket));
                while let Some(data) = stream.next().await {
                    match data? {
                        FromClient::Join { group_name } => {
                            let group = chat.get_or_create_group(group_name);
                            group.join(client.clone());
                        }
                        FromClient::Post {
                            group_name,
                            message,
                        } => match chat.get_group(&group_name) {
                            Some(group) => group.post(message),
                            None => {
                                let err = FromServer::Error(format!(
                                    "Group '{}' does not exist",
                                    group_name
                                ));
                                client.send(err).await?;
                            }
                        },
                    };
                }
                Ok::<(), ChatError>(())
            });
        }
        Ok(())
    })
}
