use crate::helper;

pub async fn solve() {
    let input = helper::get_input(5).await;
    let (stack_input, instructions_input) = input.split_once("\n\n").expect("Input must \\n\\n");

    let (mut stacks, instructions) = process_input(stack_input, instructions_input);

    let part1 = get_part_1(&mut stacks, &instructions);
    println!("part 1: {}", part1);
}

struct Instruction {
    from_stack: usize,
    to_stack: usize,
    number: usize,
}

fn process_input(
    stack_input: &str,
    instructions_input: &str,
) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let number_of_stacks = stack_input
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; number_of_stacks];

    stack_input.lines().rev().skip(1).for_each(|line| {
        for i in (1..line.len()).step_by(4) {
            let character = line.chars().nth(i).unwrap();
            if character.is_ascii_alphabetic() {
                let stack_number = (i - 1) / 4;
                stacks[stack_number].push(character);
            }
        }
    });

    let re = regex::Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Invalid regex");
    let b: Vec<Instruction> = instructions_input
        .lines()
        .map(|line| {
            let a = re.captures(line).unwrap();
            Instruction {
                number: a.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from_stack: a.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to_stack: a.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect();

    return (stacks, b);
}

fn get_part_1(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        match instruction {
            Instruction {
                from_stack,
                to_stack,
                number,
            } => {
                for _ in 0..*number {
                    let item = stacks[from_stack - 1].pop().unwrap();
                    stacks[to_stack - 1].push(item);
                }
            }
        }
    }

    stacks
        .iter()
        .filter(|stack| stack.len() > 0)
        .map(|stack| stack.last().unwrap())
        .collect()
}
