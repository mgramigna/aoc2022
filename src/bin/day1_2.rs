fn main() {
    let calories_input = include_str!("../inputs/day1.txt");

    let groups: Vec<&str> = calories_input.trim_end().split("\n\n").collect();

    let mut group_sums: Vec<usize> = groups
        .iter()
        .map(|g| {
            return g
                .split("\n")
                .map(|s| return s.parse::<usize>().expect("Could not parse int"))
                .sum();
        })
        .collect();

    group_sums.sort_by(|a, b| b.cmp(a));

    let top_three_sum: usize = group_sums.iter().take(3).sum();

    println!("{}", top_three_sum);
}
