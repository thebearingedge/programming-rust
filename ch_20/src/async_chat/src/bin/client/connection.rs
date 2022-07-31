use crate::cli;
use async_chat_lib::{
    utils::{self, ChatResult},
    FromServer,
};
use async_std::prelude::*;
use async_std::{io, net};

pub async fn send_commands(mut socket: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
         join GROUP\n\
         post GROUP MESSAGE...\n\
         Type Ctrl + D (on *nix) or Ctrl + Z (on win) \
         to close the connection."
    );
    let mut input = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = input.next().await {
        let command = match cli::parse_command_line(&(line?)) {
            Some(request) => request,
            None => continue,
        };
        utils::write_json(&mut socket, &command).await?;
        socket.flush().await?;
    }
    Ok(())
}

pub async fn handle_replies(socket: net::TcpStream) -> ChatResult<()> {
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
