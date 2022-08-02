use std::sync::Arc;
use tokio_chat_lib::FromClient;

pub fn parse_command_line(line: &str) -> Option<FromClient> {
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
