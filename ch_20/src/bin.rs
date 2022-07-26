use lib::cheapo_request;

fn main() {
    match cheapo_request("example.com", 80, "/") {
        Ok(response) => println!("{}", response),
        Err(error) => panic!("{}", error),
    }
}
