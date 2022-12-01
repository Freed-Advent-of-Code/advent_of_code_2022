mod helper;
mod day01;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    day01::solve().await;
}
