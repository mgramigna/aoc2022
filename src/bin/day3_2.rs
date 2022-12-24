use std::collections::HashSet;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(ch: char) -> usize {
    ALPHABET.find(ch).unwrap() + 1
}

fn get_common_char(group: &[&str]) -> char {
    let mut seen_group_1: HashSet<char> = HashSet::from_iter(group[0].chars());
    let seen_group_2: HashSet<char> = HashSet::from_iter(group[1].chars());
    let seen_group_3: HashSet<char> = HashSet::from_iter(group[2].chars());

    seen_group_1.retain(|ch| seen_group_2.contains(&ch) && seen_group_3.contains(&ch));

    return *seen_group_1.iter().next().unwrap();
}

fn main() {
    let rucksack_input: Vec<&str> = include_str!("../inputs/day3.txt").lines().collect();

    let groups: Vec<&[&str]> = rucksack_input.chunks(3).collect();

    let sum: usize = groups
        .iter()
        .map(|g| {
            let common_char = get_common_char(g);

            return get_priority(common_char);
        })
        .sum();

    println!("{}", sum);
}
