#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day14::solve().await;
}
