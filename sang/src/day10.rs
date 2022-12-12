use crate::helper;

pub async fn solve() {
    let input = helper::get_input(10).await;
    let part1 = helper::time_function(|| get_part_1(&input));
    println!("part 1: {}", part1);

    let part2 = helper::time_function(|| get_part_2(&input));
    println!("part 2: \n{}", part2);
    println!("===");

    match std::fs::read_to_string("input/day10-test.txt") {
        Ok(content) => println!("{}", get_part_2(&content)),
        Err(_) => panic!("invalid day10-test.txt"),
    };
}

fn get_part_1(input: &str) -> i32 {
    let cycle_marks = vec![20, 60, 100, 140, 180, 220];
    let mut cycle = 1;
    let mut register = 1;
    let mut total_strength = 0;

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "noop" => {
                if cycle_marks.contains(&cycle) {
                    total_strength += cycle * register;
                }
                cycle += 1;
            }
            "addx" => {
                let next_cycle = cycle + 1;

                if cycle_marks.contains(&next_cycle) {
                    total_strength += next_cycle * register;
                } else if cycle_marks.contains(&cycle) {
                    total_strength += cycle * register;
                }

                register += words[1].parse::<i32>().unwrap();
                cycle += 2;
            }
            _ => panic!("invalid instruction!"),
        };
    }

    total_strength
}

fn get_part_2(input: &str) -> String {
    let mut cycle: i32 = 0;
    let mut position: i32 = 1;
    let mut final_string = String::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let char = if (cycle % 40 - position).abs() <= 1 {
            '#'
        } else {
            '.'
        };

        if cycle % 40 == 0 && cycle > 1 {
            final_string.push('\n');
        }
        final_string.push(char);
        cycle += 1;

        match words[0] {
            "noop" => {}
            "addx" => {
                if cycle % 40 == 0 && cycle > 1 {
                    final_string.push('\n');
                }
                let char = if (cycle % 40 - position).abs() <= 1 {
                    '#'
                } else {
                    '.'
                };
                final_string.push(char);
                let addx = words[1].parse::<i32>().unwrap();
                position += addx;
                cycle += 1;
            }
            _ => panic!("invalid instruction"),
        }
    }
    final_string
}

#[test]
fn test_get_part_1() {
    let result = match std::fs::read_to_string("input/day10-test.txt") {
        Ok(content) => get_part_1(&content),
        Err(_) => panic!("invalid day10-test.txt"),
    };

    assert_eq!(result, 13140);
}

#[test]
fn test_get_part_2() {
    let result = match std::fs::read_to_string("input/day10-test.txt") {
        Ok(content) => get_part_2(&content),
        Err(_) => panic!("invalid day10-test.txt"),
    };
    match std::fs::read_to_string("input/day10-test-output.txt") {
        Ok(expected) => assert_eq!(result, expected),
        Err(_) => panic!("invalid day10-test-output.txt"),
    }
}
