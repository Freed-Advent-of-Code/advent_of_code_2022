use crate::helper;
use std::collections::{HashMap, HashSet};

const destination_y: i64 = 2000000;
const max_coordinate: i64 = 4000000;

pub async fn solve() {
    let input = helper::get_input(15).await;
    let part1 = helper::time_function(|| get_part_1(&input, destination_y));
    println!("part 1: {}", part1);

    let part2 = helper::time_function(|| get_part_2_a(&input, max_coordinate as usize));
    println!("part 2a: {}", part2);

    let part2 = helper::time_function(|| get_part_2(&input, max_coordinate as usize));
    println!("part 2: {}", part2);
}

#[derive(Debug)]
struct Data {
    beacons: HashSet<(i64, i64)>,
    sensors: HashSet<(i64, i64, i64)>,
}

fn process_input(input: &str) -> Data {
    let mut data = Data {
        sensors: HashSet::new(),
        beacons: HashSet::new(),
    };
    for line in input.lines() {
        let re = regex::Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
        )
        .expect("Invalid regex");

        let captures = re.captures(line).unwrap();
        let sx = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let sy = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let bx = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let by = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

        data.sensors.insert((sx, sy, get_dist((sx, sy), (bx, by))));
        data.beacons.insert((bx, by));
    }
    data
}

fn get_part_1(input: &str, target_y: i64) -> i64 {
    let data = process_input(input);
    let stack = get_intervals(&data, target_y as usize);

    let count = stack.iter().fold(0, |acc, cur| acc + (cur.1 - cur.0 + 1));

    let occupied_in_y = data
        .sensors
        .iter()
        .filter(|(_sx, sy, _dist)| *sy == target_y)
        .count() as i64
        + data
            .beacons
            .iter()
            .filter(|(_bx, by)| *by == target_y)
            .count() as i64;

    count - occupied_in_y
}

fn get_intervals(data: &Data, target_y: usize) -> Vec<(i64, i64)> {
    let mut intervals = vec![];

    for (sx, sy, dist) in &data.sensors {
        let vertical = (target_y as i64 - *sy).abs();

        if vertical > *dist {
            continue;
        }

        let horizontal = dist - vertical;
        intervals.push((*sx - horizontal, *sx + horizontal));
    }

    intervals.sort_unstable();

    let mut stack: Vec<(i64, i64)> = Vec::new();
    stack.push(intervals[0]);

    for i in 1..intervals.len() {
        if intervals[i].0 <= stack.last().unwrap().1 + 1 {
            let end = std::cmp::max(stack.last().unwrap().1, intervals[i].1);
            let n = stack.len() - 1;
            stack[n].1 = end;
        } else {
            stack.push(intervals[i]);
        }
    }

    stack
}

fn get_part_2_a(input: &str, max_coord: usize) -> i64 {
    let data = &process_input(input);

    for y in 0..max_coord + 1 {
        let intervals = get_intervals(&data, y);
        if intervals.len() == 2 {
            return y as i64 + (intervals[0].1 + 1) * max_coordinate;
        }
    }
    0
}

fn get_dist((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn get_part_2(input: &str, max_coord: usize) -> i64 {
    let data = &process_input(input);
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (sx, sy, dist) in &data.sensors {
        for dx in 0..=dist + 1 {
            let dy = dist + 1 - dx;

            for (mx, my) in directions {
                let dest_x = sx + dx * mx;
                let dest_y = sy + dy * my;
                if dest_x >= 0
                    && dest_x <= max_coord as i64
                    && dest_x >= 0
                    && dest_x <= max_coord as i64
                    && is_possible(data, dest_x, dest_y)
                {
                    println!("x, y: {}, {}", dest_x, dest_y);
                    return dest_x * max_coordinate + dest_y;
                }
            }
        }
    }
    0
}

fn is_possible(data: &Data, x: i64, y: i64) -> bool {
    for (sx, sy, dist) in &data.sensors {
        if get_dist((x, y), (*sx, *sy)) <= *dist {
            return false;
        }
    }

    true
}

#[test]
fn test_get_part_1() {
    let result = match std::fs::read_to_string("input/day15-test.txt") {
        Ok(content) => get_part_1(&content, 10),
        Err(_) => panic!("invalid day15-test.txt"),
    };

    assert_eq!(result, 26);
}

#[test]
fn test_get_part_2() {
    let result = match std::fs::read_to_string("input/day15-test.txt") {
        Ok(content) => get_part_2(&content, 20),
        Err(_) => panic!("invalid day15-test.txt"),
    };

    assert_eq!(result, 56000011);
}
