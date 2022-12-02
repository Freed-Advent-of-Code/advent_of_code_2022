use crate::helper;

pub async fn solve() {
    let input = helper::get_input(2).await;
    let part1 = input
        .lines()
        .fold(0, |accum, line| accum + get_score_1(line));

    println!("part 1: {}", part1);

    let part2 = input
        .lines()
        .fold(0, |accum, line| accum + get_score_2(line));

    println!("part 2: {}", part2);
}

fn get_score_1(line: &str) -> i32 {
    let mut result: i32 = 0;
    let bytes = line.as_bytes();

    let elf_hand = bytes[0];
    let my_hand = bytes[2] - 'X' as u8 + 'A' as u8; // A, B, or C

    // my hand: A=1, B=2, C=3
    result += (my_hand + 1 - 'A' as u8) as i32;

    if elf_hand == my_hand {
        result += 3;
    } else if elf_hand - 2 == my_hand || elf_hand + 1 == my_hand {
        result += 6;
    }
    result
}

fn get_score_2(line: &str) -> i32 {
    let mut result: i32 = 0;
}
