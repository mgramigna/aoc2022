use std::collections::HashSet;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(ch: char) -> usize {
    ALPHABET.find(ch).unwrap() + 1
}

fn get_common_char(comp_one: &str, comp_two: &str) -> char {
    let seen: HashSet<char> = HashSet::from_iter(comp_one.chars());
    let ch = comp_two.find(|c| seen.contains(&c)).unwrap();

    return comp_two.chars().nth(ch).unwrap();
}

fn main() {
    let rucksack_input: Vec<&str> = include_str!("../inputs/day3.txt").lines().collect();

    let sum: usize = rucksack_input
        .iter()
        .map(|sack| {
            let comp_one = &sack[..sack.len() / 2];
            let comp_two = &sack[sack.len() / 2..];

            let common_char = get_common_char(&comp_one, &comp_two);

            return get_priority(common_char);
        })
        .sum();

    println!("{}", sum);
}
