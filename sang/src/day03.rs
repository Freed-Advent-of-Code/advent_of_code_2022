use crate::helper;
use std::collections::HashSet;

pub async fn solve() {
    let input = helper::get_input(3).await;
    let part1 = input
        .lines()
        .fold(0, |accum, line| accum + get_part_1(line));
    println!("part 1: {}", part1);

    let mut lines = input.lines();
    let mut part2 = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        part2 += get_part_2(line1, line2, line3);
    }
    println!("part 2: {}", part2);
}

fn get_part_1(line: &str) -> i32 {
    let (first, second) = line.split_at(line.len() / 2);
    let mut seen: HashSet<i32> = HashSet::new();
    first.chars().for_each(|char| {
        seen.insert(get_value(char));
    });

    for char in second.chars() {
        if seen.contains(&get_value(char)) {
            return get_value(char);
        }
    }
    0
}

fn get_part_2(line1: &str, line2: &str, line3: &str) -> i32 {
    let seen1: HashSet<char> = HashSet::from_iter(line1.chars());
    let seen2: HashSet<char> = HashSet::from_iter(line2.chars());
    let seen3: HashSet<char> = HashSet::from_iter(line3.chars());

    for &common in seen1.intersection(&seen2) {
        if seen3.contains(&common) {
            return get_value(common);
        }
    }
    
    0
}

fn get_value(char: char) -> i32 {
    if char >= 'A' && char <= 'Z' {
        return char as i32 - 'A' as i32 + 27;
    } else if char >= 'a' && char <= 'z' {
        return char as i32 - 'a' as i32 + 1;
    }
    0
}

#[test]
fn test_get_part_1() {
    assert_eq!(get_part_1("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(get_part_1("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(get_part_1("PmmdzqPrVvPwwTWBwg"), 42);
}

#[test]
fn test_get_part_2() {
    assert_eq!(
        get_part_2(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ),
        18
    );
    assert_eq!(
        get_part_2(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ),
        52
    );
}
