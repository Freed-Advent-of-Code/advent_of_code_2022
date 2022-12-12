#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day10::solve().await;
}
