use crate::helper;

pub async fn solve() {
    let input = helper::get_input(20).await;
    let part1 = helper::time_function(|| get_part_1(&input));
    println!("part 1: {}", part1);

    let part2 = helper::time_function(|| get_part_2(&input));
    println!("part 2: {}", part2);
}

struct Number {
    order: usize,
    value: i64,
}

impl Clone for Number {
    fn clone(&self) -> Self {
        Self {
            order: self.order.clone(),
            value: self.value.clone(),
        }
    }
}

fn get_part_1(input: &str) -> i64 {
    let mut array: Vec<Number> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Number {
            order: index,
            value: line.parse::<i64>().unwrap(),
        })
        .collect();

    for i in 0..array.len() {
        move_number(&mut array, i);
    }

    get_nth_after_zero(&array, 1000)
        + get_nth_after_zero(&array, 2000)
        + get_nth_after_zero(&array, 3000)
}

fn get_part_2(input: &str) -> i64 {
    let decryption_key = 811589153;
    let mut array: Vec<Number> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Number {
            order: index,
            value: line.parse::<i64>().unwrap() * decryption_key,
        })
        .collect();

    for _ in 0..10 {
        for i in 0..array.len() {
            move_number(&mut array, i);
        }
    }

    get_nth_after_zero(&array, 1000)
        + get_nth_after_zero(&array, 2000)
        + get_nth_after_zero(&array, 3000)
}

fn move_number(array: &mut Vec<Number>, order: usize) {
    let position = array.iter().position(|num| num.order == order).unwrap();
    let number = array[position].clone();
    if number.value == 0 {
        return;
    }

    let len = array.len() as i64;

    let new_index = (((position as i64 + number.value) % (len - 1) + len - 1) % (len - 1)) as usize;
    if new_index == position {
        return;
    }

    if new_index < position {
        for i in (new_index..=position - 1).rev() {
            array[i + 1] = array[i].clone();
        }
    } else {
        for i in position + 1..=new_index {
            array[i - 1] = array[i].clone();
        }
    }
    array[new_index] = number;
}

fn get_nth_after_zero(array: &Vec<Number>, n: usize) -> i64 {
    let zero_index = array.iter().position(|n| n.value == 0).unwrap();
    let nth_position = (zero_index + n) % array.len();

    array[nth_position].value
}

#[test]
fn test_get_part_1() {
    let result = match std::fs::read_to_string("input/day20-test.txt") {
        Ok(content) => get_part_1(&content),
        Err(_) => panic!("invalid day20-test.txt"),
    };

    assert_eq!(result, 3);
}
