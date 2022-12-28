use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Correctness {
    Right,
    Wrong,
    Equal,
}

#[derive(Debug)]
enum Data {
    Int(u32),
    List(Vec<Data>),
}

fn get_ordering(pair: (&Data, &Data)) -> Correctness {
    match pair {
        (Data::Int(lhs), Data::Int(rhs)) => {
            if lhs < rhs {
                return Correctness::Right;
            } else if lhs > rhs {
                return Correctness::Wrong;
            } else {
                return Correctness::Equal;
            }
        }
        (Data::Int(lhs), Data::List(_)) => {
            let lhs_list = Data::List(vec![Data::Int(*lhs)]);

            return get_ordering((&lhs_list, pair.1));
        }
        (Data::List(_), Data::Int(rhs)) => {
            let rhs_list = Data::List(vec![Data::Int(*rhs)]);

            return get_ordering((pair.0, &rhs_list));
        }
        (Data::List(lhs), Data::List(rhs)) => {
            let mut lhs_iter = lhs.iter();
            let mut rhs_iter = rhs.iter();

            loop {
                match (lhs_iter.next(), rhs_iter.next()) {
                    (Some(l), Some(r)) => {
                        let ord = get_ordering((l, r));
                        if ord != Correctness::Equal {
                            return ord;
                        }
                    }
                    (Some(_), None) => return Correctness::Wrong,
                    (None, Some(_)) => return Correctness::Right,
                    (None, None) => return Correctness::Equal,
                }
            }
        }
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::List(Vec::new());

        let replaced = s.trim_end().replace("10", "X");
        let mut chars = replaced.chars();

        while let Some(ch) = chars.next() {
            match ch {
                ',' => {}
                '[' => {
                    let mut recursive_string = String::new();
                    let mut list_depth = 1;

                    while list_depth > 0 {
                        let next_ch = chars.next().unwrap();

                        match next_ch {
                            '[' => list_depth += 1,
                            ']' => list_depth -= 1,
                            _ => {}
                        }

                        recursive_string.push(next_ch);
                    }

                    if let Self::List(list) = &mut result {
                        let slice = &recursive_string[..recursive_string.len() - 1];
                        let parsed_recursive = slice.parse::<Self>().unwrap();
                        list.push(parsed_recursive);
                    }
                }
                ']' => {
                    return Ok(result);
                }
                'X' => {
                    if let Self::List(list) = &mut result {
                        list.push(Self::Int(10))
                    }
                }
                _ => {
                    if let Self::List(list) = &mut result {
                        list.push(Self::Int(ch.to_digit(10).unwrap()))
                    }
                }
            }
        }

        return Ok(result);
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");

    let pairs: Vec<&str> = input.split("\n\n").collect();

    let packets: Vec<(Data, Data)> = pairs
        .iter()
        .map(|p| p.split_once("\n").unwrap())
        .map(|(lhs, rhs)| {
            (
                lhs[1..].parse::<Data>().unwrap(),
                rhs[1..].parse::<Data>().unwrap(),
            )
        })
        .collect();

    let sum: usize = packets
        .iter()
        .map(|p| get_ordering((&p.0, &p.1)))
        .enumerate()
        .filter_map(|(i, o)| {
            if o == Correctness::Right {
                return Some(i + 1);
            }

            None
        })
        .sum();

    println!("{}", sum);
}
