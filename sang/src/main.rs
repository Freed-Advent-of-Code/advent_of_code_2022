#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day06::solve().await;
}
