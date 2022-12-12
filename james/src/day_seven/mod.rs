use std::fs;

struct File {
    name: String,
    size: uint32
}

struct Dir {
    name: String,
    files: Vec<File>
}

pub fn no_space_left_on_device_pt1 () {
    let f = read_input();
    println!("{}", f);

}

fn read_input () -> String {
    let file = fs::read_to_string("./src/day_seven/input.txt").unwrap();
    file
}