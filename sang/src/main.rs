#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day07::solve().await;
}
