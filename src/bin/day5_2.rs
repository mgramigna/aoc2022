use anyhow::{Error, Ok, Result};
use std::str::FromStr;

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Move {
    from_idx: usize,
    to_idx: usize,
    count: usize,
}

impl Crane {
    fn move_crate_9001(&mut self, m: &Move) {
        let from_vec = &mut self.stack[m.from_idx];
        let mut new_append_vec: Vec<char> = Vec::new();

        for _ in 0..m.count {
            let popped = from_vec.pop().expect("Should be there");
            new_append_vec.push(popped);
        }

        new_append_vec.reverse();

        self.stack[m.to_idx].append(&mut new_append_vec);
    }
}

impl FromStr for Crane {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<char>> = Vec::new();

        for line in s.lines().rev().skip(1) {
            for (idx, ch) in line.chars().enumerate() {
                if ch.is_alphanumeric() {
                    let vec_idx = idx / 4;
                    if vec_idx >= stack.len() {
                        stack.push(vec![ch])
                    } else {
                        stack[vec_idx].push(ch);
                    }
                }
            }
        }

        return Ok(Self { stack });
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Vec<usize> = s
            .split(" ")
            .filter_map(|l| l.parse::<usize>().ok())
            .collect();

        return Ok(Self {
            from_idx: m[1] - 1,
            to_idx: m[2] - 1,
            count: m[0],
        });
    }
}

fn main() {
    let (crane_input, moves_input) = include_str!("../inputs/day5.txt")
        .split_once("\n\n")
        .expect("Should work");

    let mut crane = crane_input.parse::<Crane>().unwrap();

    let move_list: Vec<Move> = moves_input
        .lines()
        .map(|l| l.parse::<Move>().unwrap())
        .collect();

    for m in move_list.iter() {
        crane.move_crate_9001(m);
    }

    let answer: String = crane
        .stack
        .iter()
        .map(|s| s.last().expect("Should exist"))
        .collect::<String>();

    println!("{:?}", answer);
}
