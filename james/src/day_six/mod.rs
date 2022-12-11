use std::fs;

pub fn turning_trouble_pt1 () -> String {

    let answer = String::from("answer!");
    answer
}

fn read_input () -> String {
    let f = fs::read_to_string("./src/day_six/input.txt").unwrap();
    f
}

fn parse_input () {
    let f = read_input();
    f
}