use crate::helper;

const SPACE: i32 = 0;
const ROCK: i32 = 1;
const SAND: i32 = 2;

pub async fn solve() {
    let input = helper::get_input(14).await;
    let mut grid_data = process_input(&input);
    let part1 = helper::time_function(|| get_part_1(&mut grid_data));
    println!("part 1: {}", part1);

    let part2 = helper::time_function(|| get_part_2(&mut grid_data));
    println!("part 2: {}", part2);
}

fn process_input(input: &str) -> GridData {
    let lines = input.lines();
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let (mut deepest, mut most_left, mut most_right) = (0, 999, 0);

    for line in lines {
        let mut prev_col = 0;
        let mut prev_row = 0;
        for (i, coordinate_str) in line.split(" -> ").enumerate() {
            let coordinate: Vec<usize> = coordinate_str
                .split(",")
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            let cur_col = coordinate[0];
            let cur_row = coordinate[1];

            if cur_row > deepest {
                deepest = cur_row;
            }
            if cur_col < most_left {
                most_left = cur_col;
            }
            if cur_col > most_right {
                most_right = cur_col;
            }

            if i != 0 {
                if cur_col == prev_col {
                    let larger = if cur_row > prev_row {
                        cur_row
                    } else {
                        prev_row
                    };
                    let smaller = if cur_row > prev_row {
                        prev_row
                    } else {
                        cur_row
                    };
                    for i in smaller..larger + 1 {
                        grid[i][cur_col] = ROCK;
                    }
                } else {
                    let larger = if cur_col > prev_col {
                        cur_col
                    } else {
                        prev_col
                    };
                    let smaller = if cur_col > prev_col {
                        prev_col
                    } else {
                        cur_col
                    };
                    for i in smaller..larger + 1 {
                        grid[cur_row][i] = ROCK;
                    }
                }
            }

            prev_col = cur_col;
            prev_row = cur_row;
        }
    }
    GridData {
        deepest,
        grid,
        most_left,
        most_right,
    }
}

fn get_part_1(grid_data: &mut GridData) -> i32 {
    let mut sand_count = 0;

    let mut grid = &mut grid_data.grid;
    while true {
        let (mut col, mut row) = (500, 0);

        while true {
            if grid[row + 1][col] == SPACE {
                row += 1;
            } else if grid[row + 1][col - 1] == SPACE {
                row += 1;
                col -= 1;
            } else if grid[row + 1][col + 1] == SPACE {
                row += 1;
                col += 1;
            } else {
                grid[row][col] = SAND;
                sand_count += 1;
                break;
            }
            if row >= grid_data.deepest || col < grid_data.most_left || col > grid_data.most_right {
                return sand_count;
            }
        }
    }
    sand_count
}

fn get_part_2(grid_data: &mut GridData) -> i32 {
    let mut sand_count = 0;

    let grid = &mut grid_data.grid;
    loop {
        let (mut col, mut row) = (500, 0);
        if grid[row][col] == SAND {
            return sand_count;
        }

        loop {
            if grid[row + 1][col] == SPACE {
                row += 1;
            } else if grid[row + 1][col - 1] == SPACE {
                row += 1;
                col -= 1;
            } else if grid[row + 1][col + 1] == SPACE {
                row += 1;
                col += 1;
            } else {
                grid[row][col] = SAND;
                sand_count += 1;
                break;
            }
            if row == grid_data.deepest + 2 {
                sand_count += 1;
                grid[row][col] = SAND;
                break;
            }
        }
    }
    sand_count
}
// 25096, 24768

fn print(grid_data: &GridData) {
    for i in 0..grid_data.grid.len() {
        for j in 0..grid_data.grid[0].len() {
            print!("{} ", grid_data.grid[i][j]);
        }
        println!("");
    }
}

#[test]
fn test_get_part_1() {
    let result = match std::fs::read_to_string("input/day14-test.txt") {
        Ok(content) => get_part_1(&mut process_input(&content)),
        Err(_) => panic!("invalid day14-test.txt"),
    };

    assert_eq!(result, 24);
}

#[test]
fn test_get_part_2() {
    let result = match std::fs::read_to_string("input/day14-test.txt") {
        Ok(content) => get_part_2(&mut process_input(&content)),
        Err(_) => panic!("invalid day14-test.txt"),
    };

    assert_eq!(result, 93);
}

struct GridData {
    grid: Vec<Vec<i32>>,
    deepest: usize,
    most_left: usize,
    most_right: usize,
}
