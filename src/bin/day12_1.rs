use std::collections::{HashSet, VecDeque};

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_neighbors(curr: &(usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    if curr.0 > 0 {
        result.push((curr.0 - 1, curr.1));
    }

    if curr.0 < rows - 1 {
        result.push((curr.0 + 1, curr.1));
    }

    if curr.1 > 0 {
        result.push((curr.0, curr.1 - 1));
    }

    if curr.1 < cols - 1 {
        result.push((curr.0, curr.1 + 1));
    }

    result
}

#[derive(Debug)]
struct Cell {
    row: usize,
    col: usize,
    dist: usize,
}

fn get_shortest_path(
    grid: &Vec<Vec<usize>>,
    start: &(usize, usize),
    end: &(usize, usize),
) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<Cell> = VecDeque::new();

    queue.push_back(Cell {
        row: start.0,
        col: start.1,
        dist: 0,
    });

    visited.insert(*start);

    while let Some(Cell { row, col, dist }) = queue.pop_front() {
        if (row, col) == *end {
            return dist as usize;
        }

        let neighbors: Vec<(usize, usize)> = get_neighbors(&(row, col), &grid);

        let curr_height: usize = grid[row][col].try_into().unwrap();
        let valid_neighbors: Vec<&(usize, usize)> = neighbors
            .iter()
            .filter(|n| {
                let neighbor_height = grid[n.0][n.1];

                return neighbor_height <= curr_height || neighbor_height == curr_height + 1;
            })
            .collect();

        for n in valid_neighbors {
            if !visited.contains(n) {
                queue.push_back(Cell {
                    row: n.0,
                    col: n.1,
                    dist: dist + 1,
                });
                visited.insert(*n);
            }
        }
    }
    panic!()
}

fn main() {
    let input = include_str!("../inputs/day12.txt");

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut grid: Vec<Vec<usize>> = vec![];

    for (row_idx, row_str) in input.lines().enumerate() {
        let mut col: Vec<usize> = vec![];
        for (col_idx, col_ch) in row_str.chars().enumerate() {
            if col_ch == 'S' {
                start = (row_idx, col_idx);
                col.push(0);
            } else if col_ch == 'E' {
                end = (row_idx, col_idx);
                col.push(25);
            } else {
                col.push(ALPHABET.find(col_ch).unwrap());
            }
        }
        grid.push(col);
    }

    let shortest_path = get_shortest_path(&grid, &start, &end);

    println!("{}", shortest_path);
}
