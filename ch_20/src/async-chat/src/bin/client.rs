use async_std::prelude::*;
use async_std::{io, net};
use chat_lib::utils::{self, ChatResult};
use chat_lib::FromClient;
use std::sync::Arc;

async fn send_commands() -> ChatResult<()> {
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
            None => {
                eprintln!("unrecognized command: {:?}", line);
                continue;
            }
        };
        println!("{:?}", command);
    }
    Ok(())
}

#[async_std::main]
async fn main() {
    let _ = send_commands().await;
}

fn parse_command_line(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;

    if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        return Some(FromClient::Post {
            group_name: Arc::new(group.into()),
            message: Arc::new(rest.trim_start().into()),
        });
    }

    if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        return Some(FromClient::Join {
            group_name: Arc::new(group.into()),
        });
    }

    None
}

fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[..space], &input[space..])),
        None => Some((input, "")),
    }
}
