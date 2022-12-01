use crate::helper;

pub async fn solve() {
    let input = helper::get_input(1).await;
    let data: i32 = input
        .rsplit("\n\n")
        .map(|x| x.rsplit("\n").collect::<Vec<&str>>())
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.parse::<i32>().unwrap()).sum())
        .reduce(|accum, item| if accum > item { accum } else { item })
        .unwrap();
    println!("{}", data);
}
