#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day05::solve().await;
}
