use async_chat_lib::utils::ChatResult;
use async_std::{net, prelude::*, task};

mod cli;
mod connection;

#[async_std::main]
async fn main() -> ChatResult<()> {
    let server_address = std::env::args().nth(1).expect("Usage: ADDRESS:PORT");
    task::block_on(async {
        let socket = net::TcpStream::connect(server_address).await?;
        socket.set_nodelay(true)?;
        connection::send_commands(socket.clone())
            .race(connection::handle_replies(socket))
            .await?;
        Ok(())
    })
}
