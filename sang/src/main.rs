#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day20::solve().await;
}
