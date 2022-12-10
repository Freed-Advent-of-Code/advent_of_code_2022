use crate::helper;
use std::collections::HashSet;

pub async fn solve() {
    let input = helper::get_input(9).await;
    let part1 = get_part_1(&input);
    println!("part 1: {}", part1);

    let part2 = get_part_2(&input);
    println!("part 2: {}", part2);
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

fn get_part_2(input: &String) -> usize {
    let mut head = Point::new();

    let mut tail1 = Point::new();
    let mut tail2 = Point::new();
    let mut tail3 = Point::new();
    let mut tail4 = Point::new();
    let mut tail5 = Point::new();
    let mut tail6 = Point::new();
    let mut tail7 = Point::new();
    let mut tail8 = Point::new();
    let mut tail9 = Point::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("0,0".to_string());

    input.lines().for_each(|line| {
        let command: Vec<&str> = line.split(" ").collect();
        for _ in 0..command[1].parse::<usize>().unwrap() {
            head.move_dir(command[0]);

            tail1.follow(&head);
            tail2.follow(&tail1);
            tail3.follow(&tail2);
            tail4.follow(&tail3);
            tail5.follow(&tail4);
            tail6.follow(&tail5);
            tail7.follow(&tail6);
            tail8.follow(&tail7);
            tail9.follow(&tail8);

            visited.insert(tail9.get_position());
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

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

trait Head {
    fn move_dir(&mut self, direction: &str) {}
}

trait Tail {
    fn follow(&mut self, head: &Point) {}
    fn get_position(&self) -> String;
}
