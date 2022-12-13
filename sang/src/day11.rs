use std::{borrow::BorrowMut, collections::HashMap};

use regex::Regex;

use crate::helper;

const ROUNDS: i32 = 20;

pub async fn solve() {
    let input = helper::get_input(11).await;
    process_input(&input);
    // let mut monkey = Monkey { items: vec![1,2], inspect_count:0, operate: Box::new(|a|a+2), divisible:2, if_true:2, if_false:1 };
}

fn process_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    /*
    input.split("\n\n").for_each(|paragraph| {
        let mut iter = paragraph.lines();
        iter.next();
        let mut split: Vec<&str> = iter.next().unwrap().split(": ").collect();
        let items: Vec<i32> = split[1]
            .split(", ")
            .map(|number| number.parse::<i32>().unwrap())
            .collect();

        let mut split: Vec<&str> = iter.next().unwrap().split("= old ").collect();
        let mut binary_op: Vec<&str> = split[1].split(" ").collect();
        let second_num: Box<i32> = Box::new(binary_op[1].parse::<i32>().unwrap_or(0));
        let mut with_self = binary_op[1].eq("old");
        let operate: Box<dyn Fn(i32) -> i32> = match binary_op[0] {
            "+" => {
                if with_self {
                    Box::new(|old: i32| old + old)
                } else {
                    Box::new(|old: i32| old + *second_num)
                }
            }
            "-" => {
                if with_self {
                    Box::new(|old: i32| 0)
                } else {
                    Box::new(|old: i32| old - *second_num)
                }
            }
            "*" => {
                if with_self {
                    Box::new(|old: i32| old * old)
                } else {
                    Box::new(|old: i32| old * *second_num)
                }
            }
            "/" => {
                if with_self {
                    Box::new(|old: i32| 1)
                } else {
                    Box::new(|old: i32| old / *second_num)
                }
            }
            _ => panic!("Invalid expression"),
        };

        let divisible = iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let if_true = iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut monkey: Monkey = Monkey {
            items,
            inspect_count: 0,
            operate,
            divisible,
            if_false,
            if_true,
        };
        monkeys.push(monkey);
    });
     */
    monkeys
}

struct Monkey {
    items: Vec<i32>,
    inspect_count: i32,
    operate: Box<dyn Fn(i32) -> i32>,
    divisible: i32,
    if_true: usize,
    if_false: usize,
}

struct Action {
    destination_monkey: usize,
    value: i32,
}

impl Monkey {
    fn throw(&self, num: i32) -> Action {
        let value = self.operate.as_ref()(num);
        let destination_monkey = if value % self.divisible == 0 {
            self.if_true
        } else {
            self.if_false
        };
        Action {
            destination_monkey,
            value,
        }
    }

    fn receive(&mut self, action: Action) {
        self.inspect_count += 1;
        self.items.push(action.value);
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}
