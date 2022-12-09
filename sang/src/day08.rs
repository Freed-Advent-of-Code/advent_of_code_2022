use crate::helper;

pub async fn solve() {
    let input = helper::get_input(8).await;
    let grid = process_input(&input);

    let part1 = helper::time_function(|| get_part_1(&grid));
    println!("part 1: {}", part1);
    
    let part2 = helper::time_function(|| get_part_2(&grid));
    println!("part 2: {}", part2);
}

fn process_input(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| (line.chars().map(|c| c.to_digit(10).unwrap() as i32)).collect())
        .collect()
}

fn get_part_1(grid: &Vec<Vec<i32>>) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut seen = vec![vec![0; cols]; rows];

    for row in 0..rows {
        let mut max_left = grid[row][0];
        let mut max_right = grid[row][cols - 1];

        for col in 0..cols {
            if grid[row][col] > max_left {
                max_left = grid[row][col];
                seen[row][col] = 1;
            }
            if grid[row][cols - 1 - col] > max_right {
                max_right = grid[row][cols - 1 - col];
                seen[row][cols - 1 - col] = 1;
            }
        }
    }

    for col in 0..cols {
        let mut max_top = grid[0][col];
        let mut max_bottom = grid[rows - 1][col];

        for row in 0..rows {
            if grid[row][col] > max_top {
                max_top = grid[row][col];
                seen[row][col] = 1;
            }
            if grid[rows - 1 - row][col] > max_bottom {
                max_bottom = grid[rows - 1 - row][col];
                seen[rows - 1 - row][col] = 1;
            }
        }
    }

    let mut result = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            result += if seen[i][j] == 1 { 1 } else { 0 };
        }
    }

    result + 2 * rows + 2 * cols - 4
}

fn get_part_2(grid: &Vec<Vec<i32>>) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut result = 0;
    let mut history = vec![vec![(0, 0, 0, 0); cols]; rows];
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let (mut top, mut left, mut bottom, mut right) = (0, 0, 0, 0);
            
            let mut vision = row - 1;
            let mut is_end: bool = false;
            while grid[vision][col] < grid[row][col] {
                top += 1;
                if vision == 0 {
                    is_end = true;
                    break;
                }
                vision -= 1;
            }
            if !is_end {
                top += 1;
            }

            vision = row + 1;
            while vision < rows && grid[vision][col] < grid[row][col] {
                vision += 1;
                bottom += 1;
            }
            if vision < rows {
                bottom += 1;
            }

            is_end = false;
            vision = col - 1;
            while grid[row][vision] < grid[row][col] {
                left += 1;
                if vision == 0 {
                    is_end = true;
                    break;
                }
                vision -= 1;
            }
            if !is_end {
                left += 1;
            }

            vision = col + 1;
            while vision < cols && grid[row][vision] < grid[row][col] {
                vision += 1;
                right += 1;
            }
            if vision < cols {
                right += 1;
            }

            if top * left * bottom * right > result {
                result = top * left * bottom * right;
                history[row][col] = (top, left, bottom, right);
            }
        }
    }
    result
}

#[test]
fn test_get_part_1() {
    let grid = match std::fs::read_to_string("input/day8-test.txt") {
        Ok(content) => process_input(&content),
        Err(_) => panic!("invalid day8-test.txt"),
    };
    assert_eq!(get_part_1(&grid), 21);
}

#[test]
fn test_get_part_2() {
    let grid = match std::fs::read_to_string("input/day8-test.txt") {
        Ok(content) => process_input(&content),
        Err(_) => panic!("invalid day8-test.txt"),
    };
    assert_eq!(get_part_2(&grid), 8);
}
