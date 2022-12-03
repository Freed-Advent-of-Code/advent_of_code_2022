use std::fs;

pub fn elves_calories() {
    let file = fs::read_to_string("./src/day_one/input.txt").unwrap();

    let result = file
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string();

    println!("{}", result);



}