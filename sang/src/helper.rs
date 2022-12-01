pub async fn get_input(day: i32) -> String {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let result = reqwest::Client::new()
        .get(url)
        .header("cookie", std::env::var("cookie").unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .trim()
        .to_string();
    result
}
