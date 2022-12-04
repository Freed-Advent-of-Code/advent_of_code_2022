use crate::helper;
use std::collections::HashSet;

pub async fn solve() {
    let input = helper::get_input(4).await;
    let part1 = input
        .lines()
        .filter(|line| {
            let vectors = parse_vectors(line);
            is_array_contained(&vectors[0], &vectors[1])
        })
        .count();
    println!("part 1: {}", part1);
}

fn parse_vectors(line: &str) -> Vec<Vec<i32>> {
    line.split(",")
        .map(|vec_str| {
            vec_str
                .split("-")
                .map(|num_str| {
                    num_str
                        .parse::<i32>()
                        .expect("wrong input: could not parse i32")
                })
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn is_array_contained(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    (is_number_contained(&first, second[0]) && is_number_contained(&first, second[1]))
        || (is_number_contained(&second, first[0]) && is_number_contained(&second, first[1]))
}

fn is_number_contained(arr: &Vec<i32>, num: i32) -> bool {
    arr[0] <= num && num <= arr[1]
}

#[test]
fn test_parse_vectors() {
    assert_eq!(parse_vectors("2-4,3-6"), vec![vec![2, 4], vec![3, 6]]);
    assert_eq!(parse_vectors("6-6,4-6"), vec![vec![6, 6], vec![4, 6]]);
}

#[test]
fn test_is_contained() {
    assert_eq!(is_array_contained(&vec![2, 4], &vec![3, 6]), false);
    assert_eq!(is_array_contained(&vec![2, 4], &vec![6, 6]), false);
    assert_eq!(is_array_contained(&vec![2, 6], &vec![6, 6]), true);
    assert_eq!(is_array_contained(&vec![2, 6], &vec![3, 5]), true);
}
