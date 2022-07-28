use async_std::prelude::*;
use async_std::{io, net, task};
use chat_lib::utils::{self, ChatResult};
use chat_lib::{FromClient, FromServer};
use std::sync::Arc;

#[async_std::main]
async fn main() -> ChatResult<()> {
    let server_address = std::env::args().nth(1).expect("Usage: ADDRESS:PORT");
    task::block_on(async {
        let socket = net::TcpStream::connect(server_address).await?;
        socket.set_nodelay(true)?;
        send_commands(socket.clone())
            .race(handle_replies(socket))
            .await?;
        Ok(())
    })
}

async fn send_commands(mut socket: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
         join GROUP\n\
         post GROUP MESSAGE...\n\
         Type Ctrl + D (on *nix) or Ctrl + Z (on win) \
         to close the connection."
    );
    let mut input = io::BufReader::new(io::stdin()).lines();
    while let Some(line_result) = input.next().await {
        let line = line_result?;
        let command = match parse_command_line(&line) {
            Some(request) => request,
            None => continue,
        };
        utils::write_json(&mut socket, &command).await?;
        socket.flush().await?;
    }
    Ok(())
}

fn parse_command_line(line: &str) -> Option<FromClient> {
    match get_next_arg(line)? {
        ("post", rest) => get_next_arg(rest).map(|(group_name, message)| FromClient::Post {
            group_name: Arc::new(group_name.into()),
            message: Arc::new(message.into()),
        }),
        ("join", rest) => {
            get_next_arg(rest)
                .filter(|(_, rest)| rest.is_empty())
                .map(|(group_name, _)| FromClient::Join {
                    group_name: Arc::new(group_name.into()),
                })
        }
        _ => None,
    }
}

fn get_next_arg(input: &str) -> Option<(&str, &str)> {
    Some(input.trim_start())
        .filter(|input| !input.is_empty())
        .map(|input| match input.find(char::is_whitespace) {
            Some(space) => (&input[..space], &input[space..]),
            None => (input, ""),
        })
}

async fn handle_replies(socket: net::TcpStream) -> ChatResult<()> {
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
