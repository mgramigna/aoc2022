fn main() {
    let calories_input = include_str!("../inputs/1_1.txt");

    let groups: Vec<&str> = calories_input.trim_end().split("\n\n").collect();

    let max_group_sum: usize = groups
        .iter()
        .map(|g| {
            return g
                .split("\n")
                .map(|s| return s.parse::<usize>().expect("Could not parse int"))
                .sum();
        })
        .max()
        .unwrap();

    println!("{}", max_group_sum);
}
