use std::fs;
use std::rc::Rc;
use std::collections::HashMap;

struct File {
    name: String,
    size: i32
}

struct Dir {
    name: String,
    size: usize,
    parent: Option<Rc<Dir>>,
    subdir: HashMAp<String, Rc<Dir>>
}

pub fn no_space_left_on_device_pt1 () {

    let f = read_input()
        .lines()
        .map(|line| {
            let parsed_line = line.split_whitespace().nth(1);
            println!("{:?}", parsed_line);
        })
        .collect::<Vec<_>>();
    println!("{:?}", f);
}

fn read_input () -> String {
    let file = fs::read_to_string("./src/day_seven/input.txt").unwrap();
    file
}