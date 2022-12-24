use anyhow::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<i8>>,
}

impl Grid {
    fn is_element_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row == self.nrows - 1 || col == self.ncols - 1 {
            return true;
        }

        let mut is_visible_top = true;
        for i in 0..row {
            if self.grid[i][col] >= self.grid[row][col] {
                is_visible_top = false;
            }
        }

        let mut is_visible_bottom = true;
        for i in row + 1..self.nrows {
            if self.grid[i][col] >= self.grid[row][col] {
                is_visible_bottom = false;
            }
        }

        let mut is_visible_left = true;
        for i in 0..col {
            if self.grid[row][i] >= self.grid[row][col] {
                is_visible_left = false;
            }
        }

        let mut is_visible_right = true;
        for i in col + 1..self.ncols {
            if self.grid[row][i] >= self.grid[row][col] {
                is_visible_right = false;
            }
        }

        return is_visible_top || is_visible_bottom || is_visible_left || is_visible_right;
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

    let mut total_visible = 0;

    let grid = input.parse::<Grid>().unwrap();

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if grid.is_element_visible(row, col) {
                total_visible += 1;
            }
        }
    }

    println!("{}", total_visible);
}
