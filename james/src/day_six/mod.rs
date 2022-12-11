use std::fs;
use itertools::Itertools;


pub fn turning_trouble_pt1 () -> String {
    let mut answer = String::new();
    let f = parse_input();
    for i in 0..f.len() - 3 {
        let l = [f[i], f[i+1], f[i+2], f[i+3]]
            .into_iter()
            .unique()
            .collect::<Vec<_>>();
        if l.len() == 4 {
            println!("the unique index is: {:?}", l);
            println!("first marker after character {}", i + 4);
            answer = (i+4).to_string();
            break
        }
    }
    answer
}
pub fn turning_trouble_pt2 () -> String {
    let mut answer = String::new();
    let f = parse_input();
    for i in 0..f.len() - 13 {
        let mut l = Vec::new();
        for ii in i..i+14 {
            l.push(f[ii]);
        }
        let t = l
            .clone()
            .into_iter()
            .unique()
            .collect::<Vec<_>>();

        if t.len() == 14 {
            println!("index is: {}", i);
            println!("vector is : {:?}", t);
            println!("first marker after character {}", i + 14);
            answer = (i+14).to_string();

            break
        }
    }
    answer
}

fn read_input () -> String {
    let f = fs::read_to_string("./src/day_six/input.txt").unwrap();
    f
}

fn parse_input () -> Vec<char> {
    let parsed_f = read_input()
        .chars()
        .collect::<Vec<char>>();
    parsed_f
}