use std::fs;
use std::str::SplitWhitespace;


fn get_stacks() -> Vec<Vec<&'static str>> {
    let mut s1 = vec!["W", "B", "D", "N", "C", "F", "J"];
    let mut s2 = vec!["P", "Z", "V", "Q", "L", "S", "T"];
    let mut s3 = vec!["P", "Z", "B", "G", "J", "T"];
    let mut s4 = vec!["D", "T", "L", "J", "Z", "B", "H", "C"];
    let mut s5 = vec!["G", "V", "B", "J", "S"];
    let mut s6 = vec!["G", "V", "B", "J", "S"];
    let mut s7 = vec!["B", "V", "D", "F", "L", "M", "P", "N"];
    let mut s8 = vec!["P", "S", "M", "F", "B", "D", "L", "R"];
    let mut s9 = vec!["V", "D", "T", "R"];

    let mut stacks = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9];

    stacks
}


pub fn supply_stacks_pt1 () {
    let stack_list = get_stacks();
    let file = fs::read_to_string("./src/day_five/input.txt").unwrap();
    let instructions = file
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|instruction| {
            let mut instruction_split = instruction.split_whitespace();
            let move_size = instruction_split.clone().nth(1).unwrap().parse::<usize>().unwrap();
            let from_container_index = instruction_split.clone().nth(3).unwrap().parse::<usize>().unwrap();
            let to_container_index = instruction_split.clone().nth(5).unwrap().parse::<usize>().unwrap();
            let mut from_container = &stack_list[&from_container_index - 1];
            let new_stack =  from_container.clone().split_off(move_size);
            new_stack

        })
        .collect::<Vec<_>>();
    println!("{:?}", stack_list);
    println!("{:?}", instructions);
}
