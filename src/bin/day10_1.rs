use std::str::FromStr;

use anyhow::Ok;

#[derive(Debug)]
enum CommandName {
    Noop,
    Addx,
}

#[derive(Debug)]
struct Command {
    name: CommandName,
    arg: Option<isize>,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, arg)) = s.split_once(" ") {
            Ok(Self {
                name: CommandName::Addx,
                arg: Some(arg.parse::<isize>().unwrap()),
            })
        } else {
            Ok(Self {
                name: CommandName::Noop,
                arg: None,
            })
        }
    }
}

fn main() {
    let input = include_str!("../inputs/day10.txt");

    let commands: Vec<Command> = input
        .lines()
        .map(|l| l.parse::<Command>().unwrap())
        .collect();

    let desired_cycle_nums: Vec<isize> = vec![20, 60, 100, 140, 180, 220];

    let mut x: isize = 1;
    let mut cycle_num: isize = 1;
    let mut total_strength = 0;

    let mut command_iter = commands.iter();
    let mut curr_command = command_iter.next();

    let mut tick: u8 = 0;
    while curr_command.is_some() {
        if desired_cycle_nums.contains(&cycle_num) {
            total_strength += cycle_num * x;
        }

        let c = curr_command.unwrap();

        if tick > 0 {
            x += c.arg.unwrap();
            curr_command = command_iter.next();

            tick = 0;
        } else {
            match c.name {
                CommandName::Noop => {
                    tick = 0;
                    curr_command = command_iter.next();
                }
                CommandName::Addx => {
                    tick = 1;
                }
            }
        }

        cycle_num += 1;
    }

    println!("{}", total_strength);
}
