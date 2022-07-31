use async_std::{net::TcpListener, prelude::*, task};
use chat_lib::utils::ChatResult;
use std::sync::Arc;

mod connection;
mod group;
mod group_table;

#[async_std::main]
async fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");
    let chat_group_table = Arc::new(group_table::GroupTable::new());
    task::block_on(async {
        let listener = TcpListener::bind(address).await?;
        while let Some(socket_result) = listener.incoming().next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async {
                connection::serve(socket, groups).await.map_err(|error| {
                    eprintln!("Error: {}", error);
                })
            });
        }
        Ok(())
    })
}
