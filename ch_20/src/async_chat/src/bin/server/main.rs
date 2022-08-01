use async_chat_lib::utils::ChatResult;
use async_std::{net::TcpListener, prelude::*, sync::Arc, task};

mod chat;
mod connection;

use chat::Chat;
use connection::{serve, Client};

#[async_std::main]
async fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");
    let chat = Arc::new(Chat::new());
    task::block_on(async {
        let listener = TcpListener::bind(address).await?;
        while let Some(socket) = listener.incoming().next().await {
            let client = Arc::new(Client::new(socket?));
            task::spawn(serve(client, chat.clone()));
        }
        Ok(())
    })
}
