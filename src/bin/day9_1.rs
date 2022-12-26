use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    count: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count_str) = s.split_once(" ").unwrap();

        let count = count_str.parse::<usize>().unwrap();

        match dir {
            "U" => Ok(Self {
                direction: Direction::Up,
                count,
            }),
            "D" => Ok(Self {
                direction: Direction::Down,
                count,
            }),
            "L" => Ok(Self {
                direction: Direction::Left,
                count,
            }),
            "R" => Ok(Self {
                direction: Direction::Right,
                count,
            }),
            _ => Err(anyhow::Error::msg("Invalid move")),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<GridItem>>,
}

#[derive(Debug, Clone, PartialEq)]
enum GridItem {
    Unvisited,
    Visited,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            grid: vec![vec![GridItem::Unvisited; size]; size],
        }
    }

    fn move_pos_once(&self, pos: &mut (isize, isize), direction: &Direction) {
        match direction {
            Direction::Up => {
                pos.0 -= 1;
            }
            Direction::Down => {
                pos.0 += 1;
            }
            Direction::Left => {
                pos.1 -= 1;
            }
            Direction::Right => {
                pos.1 += 1;
            }
        }
    }

    fn are_diagonally_touching(&self, pos1: &(isize, isize), pos2: &(isize, isize)) -> bool {
        return &(pos1.0 - 1, pos1.1 - 1) == pos2
            || &(pos1.0 - 1, pos1.1 + 1) == pos2
            || &(pos1.0 + 1, pos1.1 - 1) == pos2
            || &(pos1.0 + 1, pos1.1 + 1) == pos2;
    }

    fn move_tail(&self, head_pos: &(isize, isize), tail_pos: &mut (isize, isize)) {
        if head_pos.0 == tail_pos.0 {
            if tail_pos.1 - head_pos.1 == 2 {
                self.move_pos_once(tail_pos, &Direction::Left);
            } else if head_pos.1 - tail_pos.1 == 2 {
                self.move_pos_once(tail_pos, &Direction::Right);
            }
        } else if head_pos.1 == tail_pos.1 {
            if tail_pos.0 - head_pos.0 == 2 {
                self.move_pos_once(tail_pos, &Direction::Up)
            } else if head_pos.0 - tail_pos.0 == 2 {
                self.move_pos_once(tail_pos, &Direction::Down)
            }
        } else if !self.are_diagonally_touching(head_pos, tail_pos) {
            let vertical_direction = if tail_pos.0 < head_pos.0 {
                &Direction::Down
            } else {
                &Direction::Up
            };

            if tail_pos.1 < head_pos.1 {
                self.move_pos_once(tail_pos, &Direction::Right);
                self.move_pos_once(tail_pos, vertical_direction);
            } else {
                self.move_pos_once(tail_pos, &Direction::Left);
                self.move_pos_once(tail_pos, vertical_direction);
            }
        }
    }

    fn mark_visited(&mut self, pos: &(usize, usize)) {
        self.grid[pos.0][pos.1] = GridItem::Visited;
    }
}

fn main() {
    let size = 1000;
    let input = include_str!("../inputs/day9.txt");

    let move_list: Vec<Move> = input.lines().map(|l| l.parse::<Move>().unwrap()).collect();

    let mut grid: Grid = Grid::new(size);

    let mid = (size as isize) / 2 - 1;
    let (start_row, start_col): (isize, isize) = (mid, mid);

    let mut tail_pos = (start_row, start_col);
    let mut head_pos = (start_row, start_col);

    for m in move_list {
        for _ in 0..m.count {
            grid.move_pos_once(&mut head_pos, &m.direction);
            grid.move_tail(&head_pos, &mut tail_pos);
            grid.mark_visited(&(tail_pos.0 as usize, tail_pos.1 as usize));
        }
    }

    let visited_total: usize = grid
        .grid
        .iter()
        .flat_map(|row| row.iter().filter(|i| *i == &GridItem::Visited))
        .count();

    println!("{}", visited_total);
}
