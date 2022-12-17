#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day15::solve().await;
}
