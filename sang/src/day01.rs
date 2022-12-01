use crate::helper;
use std::collections::BinaryHeap;

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

    let mut heap: BinaryHeap<i32> = data.iter().map(|x| x.iter().sum::<i32>()).collect();

    let part2: i32 = get_max_n(&mut heap, 3).iter().sum();

    println!("part 2: {}", part2);
}

fn get_max_n(heap: &mut BinaryHeap<i32>, n: i32) -> Vec<i32> {
    let mut top3 = Vec::new();
    for _ in 0..n {
        top3.push(heap.pop().unwrap());
    }
    top3
}
