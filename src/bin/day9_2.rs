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

fn move_tail(relative_head_pos: &(isize, isize), tail_pos: &mut (isize, isize)) {
    if relative_head_pos.0 == tail_pos.0 {
        if tail_pos.1 - relative_head_pos.1 == 2 {
            move_pos_once(tail_pos, &Direction::Left);
        } else if relative_head_pos.1 - tail_pos.1 == 2 {
            move_pos_once(tail_pos, &Direction::Right);
        }
    } else if relative_head_pos.1 == tail_pos.1 {
        if tail_pos.0 - relative_head_pos.0 == 2 {
            move_pos_once(tail_pos, &Direction::Up)
        } else if relative_head_pos.0 - tail_pos.0 == 2 {
            move_pos_once(tail_pos, &Direction::Down)
        }
    } else if !are_diagonally_touching(relative_head_pos, tail_pos) {
        let vertical_direction = if tail_pos.0 < relative_head_pos.0 {
            &Direction::Down
        } else {
            &Direction::Up
        };

        if tail_pos.1 < relative_head_pos.1 {
            move_pos_once(tail_pos, &Direction::Right);
            move_pos_once(tail_pos, vertical_direction);
        } else {
            move_pos_once(tail_pos, &Direction::Left);
            move_pos_once(tail_pos, vertical_direction);
        }
    }
}

fn move_rope(movers: &mut Vec<(isize, isize)>, head_move_direction: &Direction) {
    let first_head = &mut movers[0];
    move_pos_once(first_head, head_move_direction);
    for i in 1..movers.len() {
        let (left, right) = movers.as_mut_slice().split_at_mut(i);
        let rel_head = left.last().unwrap();

        move_tail(rel_head, &mut right[0]);
    }
}

fn main() {
    let input = include_str!("../inputs/day9.txt");

    let move_list: Vec<Move> = input.lines().map(|l| l.parse::<Move>().unwrap()).collect();

    let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];

    let mut tail_visited: HashSet<(isize, isize)> = HashSet::new();

    for m in move_list {
        for _ in 0..m.count {
            move_rope(&mut rope, &m.direction);
            let last_tail_vis = rope.last().unwrap();
            tail_visited.insert((last_tail_vis.0, last_tail_vis.1));
        }
    }

    let visited_total = tail_visited.len();

    println!("{}", visited_total);
}
