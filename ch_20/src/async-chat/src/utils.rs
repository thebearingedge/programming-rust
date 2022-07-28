use async_std::io::prelude::*;
use async_std::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn write_json<W, D>(writeable: &mut W, data: &D) -> ChatResult<()>
where
    W: Write + Unpin,
    D: Serialize,
{
    let mut json = serde_json::to_string(data)?;
    json.push('\n');
    writeable.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn read_json<R, D>(readable: R) -> impl Stream<Item = ChatResult<D>>
where
    R: BufRead + Unpin,
    D: DeserializeOwned,
{
    readable.lines().map(|line_result| -> ChatResult<D> {
        let line = line_result?;
        let data = serde_json::from_str::<D>(&line)?;
        Ok(data)
    })
}
