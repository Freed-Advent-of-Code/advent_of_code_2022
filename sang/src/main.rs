mod example;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    example::get_input().await;
}
