use std::{collections::HashSet, str::FromStr};

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

fn move_pos_once(pos: &mut (isize, isize), direction: &Direction) {
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

fn are_diagonally_touching(pos1: &(isize, isize), pos2: &(isize, isize)) -> bool {
    return &(pos1.0 - 1, pos1.1 - 1) == pos2
        || &(pos1.0 - 1, pos1.1 + 1) == pos2
        || &(pos1.0 + 1, pos1.1 - 1) == pos2
        || &(pos1.0 + 1, pos1.1 + 1) == pos2;
}

fn move_tail(head_pos: &(isize, isize), tail_pos: &mut (isize, isize)) {
    if head_pos.0 == tail_pos.0 {
        if tail_pos.1 - head_pos.1 == 2 {
            move_pos_once(tail_pos, &Direction::Left);
        } else if head_pos.1 - tail_pos.1 == 2 {
            move_pos_once(tail_pos, &Direction::Right);
        }
    } else if head_pos.1 == tail_pos.1 {
        if tail_pos.0 - head_pos.0 == 2 {
            move_pos_once(tail_pos, &Direction::Up)
        } else if head_pos.0 - tail_pos.0 == 2 {
            move_pos_once(tail_pos, &Direction::Down)
        }
    } else if !are_diagonally_touching(head_pos, tail_pos) {
        let vertical_direction = if tail_pos.0 < head_pos.0 {
            &Direction::Down
        } else {
            &Direction::Up
        };

        if tail_pos.1 < head_pos.1 {
            move_pos_once(tail_pos, &Direction::Right);
            move_pos_once(tail_pos, vertical_direction);
        } else {
            move_pos_once(tail_pos, &Direction::Left);
            move_pos_once(tail_pos, vertical_direction);
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day9.txt");

    let move_list: Vec<Move> = input.lines().map(|l| l.parse::<Move>().unwrap()).collect();

    let (start_row, start_col): (isize, isize) = (0, 0);

    let mut tail_pos = (start_row, start_col);
    let mut head_pos = (start_row, start_col);

    let mut tail_visited: HashSet<(isize, isize)> = HashSet::new();

    for m in move_list {
        for _ in 0..m.count {
            move_pos_once(&mut head_pos, &m.direction);
            move_tail(&head_pos, &mut tail_pos);
            tail_visited.insert((tail_pos.0, tail_pos.1));
        }
    }

    let visited_total = tail_visited.len();

    println!("{}", visited_total);
}
