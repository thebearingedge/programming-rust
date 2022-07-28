use async_std::io::prelude::*;
use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<S, D>(socket: &mut S, data: &D) -> ChatResult<()>
where
    S: Write + Unpin,
    D: Serialize,
{
    let mut json = serde_json::to_string(data)?;
    json.push('\n');
    socket.write_all(json.as_bytes()).await?;
    Ok(())
}

pub async fn receive_as_json<S, D>(socket: S) -> impl Stream<Item = ChatResult<D>>
where
    S: BufRead + Unpin,
    D: DeserializeOwned,
{
    socket.lines().map(|line_result| -> ChatResult<D> {
        let line = line_result?;
        let data = serde_json::from_str::<D>(&line)?;
        Ok(data)
    })
}
