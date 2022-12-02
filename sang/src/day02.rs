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
    let bytes = line.as_bytes();
    // A=1, B=2, C=3
    let elf_hand = bytes[0] as i32 - 'A' as i32 + 1;
    let my_hand: i32;

    // 0 = loss, 1 = draw, 2 = win
    let outcome = bytes[2] - 'X' as u8;
    result += (outcome as i32) * 3;

    if outcome == 1 {
        my_hand = elf_hand;
    } else if outcome == 0 {
        // loss
        my_hand = (elf_hand + 2) % 3;
    } else {
        // win
        my_hand = (elf_hand + 1) % 3;
    }

    if my_hand == 0 {
        result += 3;
    } else {
        result += my_hand;
    }

    result
}
