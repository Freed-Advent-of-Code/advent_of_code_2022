#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day04::solve().await;
}
