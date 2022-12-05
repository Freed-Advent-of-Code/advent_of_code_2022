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
                    pair
                        .split("-")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .map(|p| p[0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", list);


}