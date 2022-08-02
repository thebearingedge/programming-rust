use crate::cli;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::tcp::{ReadHalf, WriteHalf},
};
use tokio_chat_lib::{
    utils::{self, ChatResult},
    FromServer,
};
use tokio_stream::StreamExt;

pub async fn send_commands(mut socket: WriteHalf<'_>) -> ChatResult<()> {
    println!(
        "Commands:\n\
         join GROUP\n\
         post GROUP MESSAGE...\n\
         Type Ctrl + D (on *nix) or Ctrl + Z (on win) \
         to close the connection."
    );
    let mut input = BufReader::new(io::stdin()).lines();
    while let Some(line) = input.next_line().await? {
        let command = match cli::parse_command_line(&line) {
            Some(request) => request,
            None => continue,
        };
        utils::write_json(&mut socket, &command).await?;
        socket.flush().await?;
    }
    Ok(())
}

pub async fn handle_replies(socket: ReadHalf<'_>) -> ChatResult<()> {
    let mut stream = utils::read_json(io::BufReader::new(socket));
    while let Some(data) = stream.next().await {
        match data? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => {
                println!("error from server: {}", message);
            }
        }
    }
    Ok(())
}
