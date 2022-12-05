use crate::helper;

pub async fn solve() {
    let input = helper::get_input(2).await;
    let part1 = input
        .lines()
        .fold(0, |accum, line| accum + get_score_1(line));

    println!("part 1: {}", part1);

    let part2 = input
        .lines()
        .fold(0, |accum, line| accum + get_score_2(line));

    println!("part 2: {}", part2);
}

fn get_score_1(line: &str) -> i32 {
    let mut result: i32 = 0;

    let elf_hand = match line.chars().nth(0).unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        _ => Hand::Scissors,
    };
    let my_hand = match line.chars().nth(2).unwrap() {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        _ => Hand::Scissors,
    };

    result += my_hand.get_value();
    result += my_hand.plays_against(elf_hand).get_value();
    result
}

fn get_score_2(line: &str) -> i32 {
    let mut result: i32 = 0;
    let elf_hand = match line.chars().nth(0).unwrap() {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        _ => Hand::Scissors,
    };

    let my_hand = match line.chars().nth(2).unwrap() {
        'X' => elf_hand.wins_against(),
        'Y' => elf_hand.draws_against(),
        _ => elf_hand.loses_against(),
    };
    result += my_hand.get_value();
    result += my_hand.plays_against(elf_hand).get_value();
    result
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}
#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

trait Playable {
    fn loses_against(&self) -> Hand;

    fn wins_against(&self) -> Hand;

    fn draws_against(&self) -> Hand;

    fn plays_against(&self, hand: Hand) -> Outcome;
}

impl Playable for Hand {
    fn draws_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissors => Hand::Scissors,
        }
    }

    fn wins_against(&self) -> Hand {
        match self {
            Hand::Paper => Hand::Rock,
            Hand::Rock => Hand::Scissors,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses_against(&self) -> Hand {
        match self {
            Hand::Paper => Hand::Scissors,
            Hand::Rock => Hand::Paper,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn plays_against(&self, hand: Hand) -> Outcome {
        if self.draws_against() == hand {
            return Outcome::Draw;
        }
        match self {
            Hand::Paper => {
                if matches!(hand, Hand::Scissors) {
                    Outcome::Loss
                } else {
                    Outcome::Win
                }
            }
            Hand::Scissors => {
                if matches!(hand, Hand::Rock) {
                    Outcome::Loss
                } else {
                    Outcome::Win
                }
            }
            Hand::Rock => {
                if matches!(hand, Hand::Paper) {
                    Outcome::Loss
                } else {
                    Outcome::Win
                }
            }
        }
    }
}

trait Value {
    fn get_value(&self) -> i32;
}

impl Value for Hand {
    fn get_value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Value for Outcome {
    fn get_value(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[test]
fn test_hand() {
    let paper = Hand::Paper;

    assert!(paper.loses_against() == Hand::Scissors);
    assert!(paper.wins_against() == Hand::Rock);
    assert!(paper.draws_against() == Hand::Paper);
}

#[test]
fn test_outcome() {
    let paper = Hand::Paper;

    assert!(paper.plays_against(Hand::Scissors) == Outcome::Loss);
    assert!(paper.plays_against(Hand::Paper) == Outcome::Draw);
    assert!(paper.plays_against(Hand::Rock) == Outcome::Win);
}

#[test]
fn test_score_2() {
    let line = "A Y";
    assert_eq!(get_score_2(line), 4);

    let line = "B X";
    assert_eq!(get_score_2(line), 1);

    let line = "C Z";
    assert_eq!(get_score_2(line), 7);
}
