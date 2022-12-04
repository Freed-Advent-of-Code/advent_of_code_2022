use std::fs;

fn read_file () -> String {
    fs::read_to_string("./src/day_four/input.txt").unwrap()
}

pub fn pairs_part_one() {
    let file = read_file();

    let list = file.lines().map(|l| {
        let v = l.replace("-", ",");
        v

    })
        .collect::<Vec<_>>();

    println!("{:?}", list);


}