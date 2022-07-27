use futures;
use surf;

async fn make_many_requests() -> Vec<surf::Result<String>> {
    let urls = vec![
        "http://example.com",
        "https://jsonplaceholder.typicode.com/users/1",
        "https://pokeapi.co/api/v2/pokemon/bulbasaur",
    ];

    let client = surf::Client::new();

    futures::future::join_all(urls.iter().map(|url| {
        let req = client.get(url).recv_string();
        async_std::task::spawn(req)
    }))
    .await
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
