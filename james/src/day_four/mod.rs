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
                .into_iter()
                .map(|pair| {
                    pair
                        .split("-")
                        .into_iter()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", list);


}