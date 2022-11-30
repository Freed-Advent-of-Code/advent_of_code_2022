pub async fn get_input() {
    let result = reqwest::Client::new().get("https://adventofcode.com/2021/day/2/input")
    .header("cookie", std::env::var("cookie").unwrap())
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    println!("res: {}", result);    
}