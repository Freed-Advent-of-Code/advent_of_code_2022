use std::fs;

fn read_file () -> String {
    fs::read_to_string("./src/day_four/input.txt").unwrap()
}

pub fn pairs_part_one() {
    let file = read_file();

    let list = file
        .lines()
        .map(|pairs| {
            pairs
                .split(",")
                .map(|pair| {
                    pair.split("-").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .filter(|u|{
            (u[0] <= u[2] && u[1] >= u[3]) || (u[2] <= u[0] && u[3] >= u[1])
        })
        .count();

    println!("{:?}", list);


}