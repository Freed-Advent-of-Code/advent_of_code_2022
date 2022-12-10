use crate::helper;
use std::collections::HashSet;

pub async fn solve() {
    let input = helper::get_input(9).await;
    let part1 = get_part_1(&input);
    println!("part 1: {}", part1);
}

fn get_part_1(input: &String) -> usize {
    let mut head = Point::new();
    let mut tail = Point::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0,0".to_string());
    input.lines().for_each(|line| {
        let command: Vec<&str> = line.split(" ").collect();
        for _ in 0..command[1].parse::<usize>().unwrap() {
            head.move_dir(command[0]);
            tail.follow(&head);
            visited.insert(tail.get_position());
        }
    });
    visited.len()
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl Head for Point {
    fn move_dir(&mut self, direction: &str) {
        match direction {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!("unexpected input"),
        }
    }
}

impl Tail for Point {
    fn follow(&mut self, head: &Point) {
        if (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1 {
            return;
        }
        if self.x - head.x == 2 {
            self.x -= 1;
            if self.y > head.y {
                self.y -= 1;
            } else if self.y < head.y {
                self.y += 1;
            }
        } else if self.x - head.x == -2 {
            self.x += 1;
            if self.y > head.y {
                self.y -= 1;
            } else if self.y < head.y {
                self.y += 1;
            }
        } else if self.y - head.y == 2 {
            self.y -= 1;
            if self.x > head.x {
                self.x -= 1;
            } else if self.x < head.x {
                self.x += 1;
            }
        } else if self.y - head.y == -2 {
            self.y += 1;
            if self.x > head.x {
                self.x -= 1;
            } else if self.x < head.x {
                self.x += 1;
            }
        }
    }

    fn get_position(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

trait Head {
    fn move_dir(&mut self, direction: &str) {}
}

trait Tail {
    fn follow(&mut self, head: &Point) {}
    fn get_position(&self) -> String;
}
