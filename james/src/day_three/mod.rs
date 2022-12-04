use std::collections::HashMap;
use std::fs;

pub fn compartment_part_one () -> String {

    let file = fs::read_to_string("./src/day_three/input.txt").unwrap();

    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

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