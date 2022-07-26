use async_std::net;
use async_std::prelude::*;

async fn cheapo_request(host: &str, port: u16, path: &str) -> async_std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();

    socket.read_to_string(&mut response).await?;

    Ok(response)
}

async fn make_many_requests() -> Vec<std::io::Result<String>> {
    let requests = vec![
        ("example.com", 80, "/"),
        ("example.com", 80, "/rats"),
        ("example.com", 80, "/horses"),
    ];

    let mut pending = vec![];

    for (host, port, path) in requests {
        pending.push(async_std::task::spawn_local(cheapo_request(host, port, path)).await);
    }

    pending
}

#[async_std::main]
async fn main() {
    for result in make_many_requests().await {
        match result {
            Ok(response) => println!("{}", response),
            Err(error) => eprintln!("{}", error),
        }
    }
}
