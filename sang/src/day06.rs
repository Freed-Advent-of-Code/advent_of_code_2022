use crate::helper;
use std::collections::HashMap;

pub async fn solve() {
    let input = helper::get_input(6).await;
    let part1 = helper::time_function(|| get_part_1(&input));
    println!("part 1: {}", part1);
}

fn get_part_1(input: &str) -> usize {
    let mut seen: HashMap<char, usize> = HashMap::new();

    for (i, ch) in input.chars().enumerate() {
        seen.insert(ch, 1 + if seen.contains_key(&ch) { seen[&ch] } else { 0 });

        if i >= 3 {
            if seen.keys().count() == 4 {
                return i + 1;
            }

            let remove_char = input.chars().nth(i - 3).unwrap();
            if let Some(count) = seen.get(&remove_char) {
                match count {
                    1 => seen.remove(&remove_char),
                    _ => seen.insert(remove_char, count - 1),
                };
            }
        }
    }
    0
}

#[test]
fn test_get_part_1() {
    assert_eq!(get_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(get_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(get_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(get_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(get_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}
