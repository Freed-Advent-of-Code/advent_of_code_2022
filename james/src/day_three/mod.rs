use std::collections::HashMap;
use std::fs;

fn read_file () -> String {
    fs::read_to_string("./src/day_three/input.txt").unwrap()
}

fn get_letter_scores () -> HashMap<char, usize> {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
    letter_scores
}

pub fn compartment_part_one () -> String {
    let file = read_file();

    let letter_scores = get_letter_scores();

    let list: usize =  file
        .lines()
        .map(|compartment| {
            let v = compartment.to_string();
            let string_length = v.len()/2;
            let (first, second)= v.split_at(string_length);
            let same_char = first
                .chars()
                .find(|c| second.contains(*c))
                .unwrap();
            letter_scores.get(&same_char).unwrap()
        })
        .sum();
    list.to_string()
}

pub fn compartment_part_two() -> String {

    let file = read_file();
    let letter_scores = get_letter_scores();

    let list: usize = file
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| {
            let one = x[0];
            let two = x[1];
            let three = x[2];
            let same_char = one.chars().find(|one_char| {
                two.contains(*one_char) && three.contains(*one_char)
            })
                .unwrap();
            letter_scores.get(&same_char).unwrap()
        })
        .sum::<usize>();

    list.to_string()
}