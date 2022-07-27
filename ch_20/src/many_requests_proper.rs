use surf;

async fn make_many_requests() -> Vec<surf::Result<String>> {
    let urls = vec![
        "http://example.com",
        "https://jsonplaceholder.typicode.com/users/1",
        "https://pokeapi.co/api/v2/pokemon/bulbasaur",
    ];

    let client = surf::Client::new();

    let mut handles = vec![];

    for url in urls {
        let request = client.get(url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut responses = vec![];

    for handle in handles {
        responses.push(handle.await);
    }

    responses
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
