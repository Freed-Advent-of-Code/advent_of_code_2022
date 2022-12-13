#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day11::solve().await;
}
