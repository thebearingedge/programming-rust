use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_chat_lib::utils::ChatResult;

mod chat;
mod connection;

use chat::Chat;
use connection::serve;

#[tokio::main]
async fn main() -> ChatResult<()> {
    let server_address = std::env::args().nth(1).expect("Usage: server ADDRESS:PORT");
    let listener = TcpListener::bind(server_address).await?;
    let chat = Arc::new(Chat::new());
    loop {
        let (socket, _) = listener.accept().await?;
        let chat = chat.clone();
        tokio::spawn(async { serve(chat, socket).await });
    }
}
