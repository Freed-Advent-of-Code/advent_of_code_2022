use crate::helper;
use std::collections::VecDeque;

pub async fn solve() {
    let input = helper::get_input(12).await;
    let part1 = helper::time_function(|| get_part_1(&input));
    println!("part 1: {}", part1);
}

fn process_input(input: &str) -> Graph {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut grid: Vec<Vec<i32>> = vec![];

    for (i, line) in input.lines().enumerate() {
        grid.push(vec![0; line.len()]);
        for (j, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (i, j);
                grid[i][j] = 0;
            } else if char == 'E' {
                end = (i, j);
                grid[i][j] = 25;
            } else {
                grid[i][j] = char as i32 - 'a' as i32;
            }
        }
    }

    Graph { grid, start, end }
}

fn bfs(graph: &Graph) -> i32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; graph.grid[0].len()]; graph.grid.len()];
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((graph.start.0, graph.start.1, 0));

    let (max_row, max_col) = (graph.grid.len(), graph.grid[0].len());

    while !queue.is_empty() {
        let (i, j, dist) = queue.pop_front().unwrap();
        if visited[i][j] {
            continue;
        }
        if (i, j) == graph.end {
            return dist;
        }
        visited[i][j] = true;

        if i > 0 && graph.grid[i][j] + 1 >= graph.grid[i - 1][j] {
            queue.push_back((i - 1, j, dist + 1));
        }
        if i < max_row - 1 && graph.grid[i][j] + 1 >= graph.grid[i + 1][j] {
            queue.push_back((i + 1, j, dist + 1));
        }
        if j > 0 && graph.grid[i][j] + 1 >= graph.grid[i][j - 1] {
            queue.push_back((i, j - 1, dist + 1));
        }
        if j < max_col - 1 && graph.grid[i][j] + 1 >= graph.grid[i][j + 1] {
            queue.push_back((i, j + 1, dist + 1));
        }
    }
    0
}

fn get_part_1(input: &str) -> i32 {
    let graph = process_input(&input);
    bfs(&graph)
}

struct Graph {
    grid: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[test]
fn test_get_part_1() {
    let result = match std::fs::read_to_string("input/day12-test.txt") {
        Ok(content) => get_part_1(&content),
        Err(_) => panic!("invalid day12-test.txt"),
    };

    assert_eq!(result, 31);
}
