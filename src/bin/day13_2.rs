use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Data {
    Int(u32),
    List(Vec<Data>),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Data::Int(lhs), Data::Int(rhs)) => return lhs.partial_cmp(rhs),
            (Data::Int(lhs), Data::List(_)) => {
                let lhs_list = Data::List(vec![Data::Int(*lhs)]);

                return Some(lhs_list.partial_cmp(other).unwrap());
            }
            (Data::List(_), Data::Int(rhs)) => {
                let rhs_list = Data::List(vec![Data::Int(*rhs)]);

                return Some(self.partial_cmp(&rhs_list).unwrap());
            }
            (Data::List(lhs), Data::List(rhs)) => {
                let mut lhs_iter = lhs.iter();
                let mut rhs_iter = rhs.iter();

                loop {
                    match (lhs_iter.next(), rhs_iter.next()) {
                        (Some(l), Some(r)) => {
                            if let Some(ord) = l.partial_cmp(r) {
                                if ord != Ordering::Equal {
                                    return Some(ord);
                                }
                            }
                        }
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                    }
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

    let pairs: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let mut packets: Vec<Data> = pairs
        .iter()
        .map(|p| p[1..].parse::<Data>().unwrap())
        .collect();

    packets.append(&mut vec![
        Data::List(vec![Data::List(vec![Data::Int(2)])]),
        Data::List(vec![Data::List(vec![Data::Int(6)])]),
    ]);

    packets.sort();

    let i = packets
        .iter()
        .position(|p| *p == Data::List(vec![Data::List(vec![Data::Int(2)])]))
        .unwrap();

    let j = packets
        .iter()
        .position(|p| *p == Data::List(vec![Data::List(vec![Data::Int(6)])]))
        .unwrap();

    println!("{}", (i + 1) * (j + 1));
}
