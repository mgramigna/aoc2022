use std::str::FromStr;

#[derive(Debug)]
enum Operand {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    operand: Operand,
    rhs: Option<usize>,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by_test: usize,
    true_condition_monkey: usize,
    false_condition_monkey: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().map(str::trim).skip(1).collect();

        let mut starting_items: Vec<usize> = vec![];
        let mut operation: Operation = Operation {
            operand: Operand::Multiply,
            rhs: Some(1),
        };
        let mut divisible_by_test: usize = 0;
        let mut true_condition_monkey: usize = 0;
        let mut false_condition_monkey: usize = 0;

        for l in lines {
            if l.starts_with("Starting items: ") {
                let (_, item_list_str) = l.split_once("Starting items: ").unwrap();
                starting_items = item_list_str
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            } else if l.starts_with("Operation: ") {
                let (_, expr) = l.split_once(" = ").unwrap();
                let expr_pieces: Vec<&str> = expr.split(" ").collect();

                let operand: Operand = match expr_pieces[1] {
                    "+" => Ok(Operand::Add),
                    "*" => Ok(Operand::Multiply),
                    _ => Err(anyhow::Error::msg("Unkown operand")),
                }
                .unwrap();

                if let Ok(rhs) = expr_pieces[2].parse::<usize>() {
                    operation = Operation {
                        operand,
                        rhs: Some(rhs),
                    }
                } else {
                    operation = Operation { operand, rhs: None }
                }
            } else if l.starts_with("Test: ") {
                let divisible_by_pieces: Vec<&str> = l.split(" ").collect();
                divisible_by_test = divisible_by_pieces.last().unwrap().parse().unwrap();
            } else if l.starts_with("If true: ") {
                let (_, monkey_idx_str) = l.split_once("throw to monkey ").unwrap();
                true_condition_monkey = monkey_idx_str.parse().unwrap();
            } else if l.starts_with("If false: ") {
                let (_, monkey_idx_str) = l.split_once("throw to monkey ").unwrap();
                false_condition_monkey = monkey_idx_str.parse().unwrap();
            }
        }

        Ok(Self {
            items: starting_items,
            operation,
            divisible_by_test,
            true_condition_monkey,
            false_condition_monkey,
        })
    }
}

fn main() {
    let input = include_str!("../inputs/day11.txt");

    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect();

    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let lcm: usize = monkeys.iter().map(|m| m.divisible_by_test).product();

    let num_rounds = 10000;

    for _ in 0..num_rounds {
        for monkey_idx in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[monkey_idx].items);
            inspections[monkey_idx] += items.len();
            for mut item in items {
                match monkeys[monkey_idx].operation.operand {
                    Operand::Add => {
                        if let Some(rhs) = monkeys[monkey_idx].operation.rhs {
                            item += rhs;
                        } else {
                            item += item;
                        }
                    }
                    Operand::Multiply => {
                        if let Some(rhs) = monkeys[monkey_idx].operation.rhs {
                            item *= rhs;
                        } else {
                            item *= item;
                        }
                    }
                }

                item %= lcm;

                if item % monkeys[monkey_idx].divisible_by_test == 0 {
                    let dst_monkey_idx = monkeys[monkey_idx].true_condition_monkey;
                    monkeys[dst_monkey_idx].items.push(item);
                } else {
                    let dst_monkey_idx = monkeys[monkey_idx].false_condition_monkey;
                    monkeys[dst_monkey_idx].items.push(item);
                }
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));

    let monkey_business: usize = inspections.iter().take(2).product();

    println!("{}", monkey_business);
}
