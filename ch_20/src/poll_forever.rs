use async_std::future;

#[async_std::main]
async fn main() {
    future::pending::<()>().await;
}
