use crate::helper;

pub async fn solve() {
    let input = helper::get_input(1).await;
    let data: Vec<Vec<i32>> = input
        .rsplit("\n\n")
        .map(|x| x.rsplit("\n").collect::<Vec<&str>>())
        .map(|x| {
            x.into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let part1 = data
        .iter()
        .map(|x| x.iter().sum::<i32>())
        .reduce(|accum, item| if accum > item { accum } else { item })
        .unwrap();
    println!("part 1: {}", part1);

    let mut sums: Vec<i32> = data.iter().map(|x| x.iter().sum::<i32>()).collect();
    sums.sort_unstable_by(|a, b| b.cmp(a));

    let part2 = sums[0] + sums[1] + sums[2];
    println!("part 2: {}", part2);
}
