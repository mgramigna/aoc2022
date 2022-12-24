use anyhow::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<i8>>,
}

impl Grid {
    fn get_scenic_score(&self, row: usize, col: usize) -> usize {
        let mut count_top = 0;
        for i in (0..row).rev() {
            count_top += 1;

            if self.grid[i][col] >= self.grid[row][col] {
                break;
            }
        }

        let mut count_bottom = 0;
        for i in row + 1..self.nrows {
            count_bottom += 1;

            if self.grid[i][col] >= self.grid[row][col] {
                break;
            }
        }
        let mut count_left = 0;
        for i in (0..col).rev() {
            count_left += 1;

            if self.grid[row][i] >= self.grid[row][col] {
                break;
            }
        }
        let mut count_right = 0;
        for i in col + 1..self.ncols {
            count_right += 1;

            if self.grid[row][i] >= self.grid[row][col] {
                break;
            }
        }

        return count_top * count_bottom * count_left * count_right;
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<i8>> = s
            .lines()
            .map(|l| {
                l.trim()
                    .split("")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i8>().unwrap())
                    .collect()
            })
            .collect();

        let nrows = grid.len();
        let ncols = grid[0].len();

        return Ok(Self { ncols, nrows, grid });
    }
}

fn main() {
    let input = include_str!("../inputs/day8.txt");

    let grid = input.parse::<Grid>().unwrap();

    let mut scores: Vec<usize> = vec![];

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            scores.push(grid.get_scenic_score(row, col));
        }
    }

    let max = scores.iter().max().unwrap();

    println!("{}", max);
}
