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

    let row_end_cycles: Vec<usize> = vec![40, 80, 120, 160, 200, 240];

    let mut result = String::from("");
    let mut sprite_pos: Vec<bool> = vec![false; 40];

    sprite_pos[0..3].fill(true);

    let mut x: isize = 1;
    let mut cycle_num: usize = 1;
    let mut tick: u8 = 0;
    let mut current_col = 0;

    let mut command_iter = commands.iter();
    let mut curr_command = command_iter.next();

    while curr_command.is_some() {
        if sprite_pos[current_col] {
            result += "#";
        } else {
            result += ".";
        }

        let c = curr_command.unwrap();

        if tick > 0 {
            if x - 1 >= 0 {
                sprite_pos[(x - 1) as usize] = false;
            }

            if x >= 0 {
                sprite_pos[x as usize] = false;
            }

            if x + 1 >= 0 {
                sprite_pos[(x + 1) as usize] = false;
            }

            x += c.arg.unwrap();

            if x - 1 >= 0 && x - 1 < 39 {
                sprite_pos[(x - 1) as usize] = true;
            }

            if x >= 0 && x < 39 {
                sprite_pos[x as usize] = true;
            }

            if x + 1 >= 0 && x + 1 < 39 {
                sprite_pos[(x + 1) as usize] = true;
            }

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

        if row_end_cycles.contains(&cycle_num) {
            current_col = 0;
            result += "\n";
        } else {
            current_col += 1;
        }

        cycle_num += 1;
    }

    println!("{}", result);
}
