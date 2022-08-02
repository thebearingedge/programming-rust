use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio_stream::{wrappers::LinesStream, Stream, StreamExt};

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn write_json<W, D>(writeable: &mut W, data: &D) -> ChatResult<()>
where
    W: AsyncWrite + Unpin,
    D: Serialize,
{
    let mut json = serde_json::to_string(data)?;
    json.push('\n');
    writeable.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn read_json<R, D>(readable: R) -> impl Stream<Item = ChatResult<D>>
where
    R: AsyncBufRead + Unpin,
    D: DeserializeOwned,
{
    LinesStream::new(readable.lines()).map(|line_result| {
        let line = line_result?;
        let data = serde_json::from_str::<D>(&line)?;
        Ok(data)
    })
}
