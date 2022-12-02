mod helper;
mod day02;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    day02::solve().await;
}
