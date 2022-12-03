mod helper;
mod day03;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    day03::solve().await;
}
