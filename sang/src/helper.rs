use std::time::SystemTime;

pub async fn get_input(day: i32) -> String {
    let file_name = &format!("input/day{}.txt", day);

    match std::fs::read_to_string(file_name) {
        Ok(content) => return content,
        Err(_) => {
            let result = get_data(day).await;
            std::fs::write(file_name, &result).expect("couldn't write to file");
            result
        }
    }
}

pub async fn get_data(day: i32) -> String {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let result = reqwest::Client::new()
        .get(url)
        .header(
            "cookie",
            std::env::var("cookie").expect("env variable 'cookie' is not found"),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    result
}

pub fn time_function<F: FnMut() -> T, T>(mut function: F) -> T {
    let start = SystemTime::now();
    let result = function();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("It took {} microseconds.", duration.as_micros());
    result
}
