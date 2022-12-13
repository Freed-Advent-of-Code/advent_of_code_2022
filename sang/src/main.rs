#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day12::solve().await;
}
