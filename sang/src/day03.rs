use crate::helper;
use std::collections::HashSet;

pub async fn solve() {
    let input = helper::get_input(3).await;
    let part1 = input
        .lines()
        .fold(0, |accum, line| accum + get_priority(line));
    println!("part 1: {}", part1);
}

fn get_priority(line: &str) -> i32 {
    let (first, second) = line.split_at(line.len() / 2);
    let mut seen: HashSet<i32> = HashSet::new();
    first.chars().into_iter().for_each(|char| {
        seen.insert(get_value(char));
    });

    for char in second.chars() {
        if seen.contains(&get_value(char)) {
            return get_value(char);
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
fn test_get_priority() {
    assert_eq!(get_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(get_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(get_priority("PmmdzqPrVvPwwTWBwg"), 42);
}
