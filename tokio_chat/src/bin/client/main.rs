use tokio::{join, net::TcpStream};
use tokio_chat_lib::utils::ChatResult;

mod cli;
mod connection;

#[tokio::main]
async fn main() -> ChatResult<()> {
    let server_address = std::env::args().nth(1).expect("Usage: ADDRESS:PORT");
    let mut socket = TcpStream::connect(server_address).await?;
    socket.set_nodelay(true)?;
    let (read, write) = socket.split();
    let _ = join!(
        connection::handle_replies(read),
        connection::send_commands(write),
    );
    Ok(())
}
