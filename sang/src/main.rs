#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day08::solve().await;
}
