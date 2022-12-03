#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day03::solve().await;
}
