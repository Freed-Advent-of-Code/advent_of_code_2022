#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    sang::day09::solve().await;
}
