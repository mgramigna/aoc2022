use linked_hash_set::LinkedHashSet;

fn find_marker(msg: &str) -> usize {
    let mut seen: LinkedHashSet<char> = LinkedHashSet::new();
    for (i, ch) in msg.chars().enumerate() {
        if seen.len() < 4 {
            while seen.contains(&ch) {
                seen.pop_front();
            }
            seen.insert(ch);
        }

        if seen.len() == 4 {
            return i + 1;
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("../inputs/day6.txt");

    println!("{}", find_marker(input));
}
