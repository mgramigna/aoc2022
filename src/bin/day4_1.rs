fn has_overlapping_interval(assignment: &str) -> bool {
    let sections: Vec<&str> = assignment.split(",").collect();

    let ranges: Vec<(usize, usize)> = sections
        .iter()
        .map(|s| {
            let range: Vec<&str> = s.split("-").collect();

            return (
                range[0].parse::<usize>().unwrap(),
                range[1].parse::<usize>().unwrap(),
            );
        })
        .collect();

    let range_a = ranges[0];
    let range_b = ranges[1];

    if range_a.1 == range_b.1 {
        return true;
    }

    if range_a.1 > range_b.1 {
        return range_a.0 <= range_b.0;
    }

    return range_b.0 <= range_a.0;
}

fn main() {
    let assignments: Vec<&str> = include_str!("../inputs/day4.txt").lines().collect();

    let count: usize = assignments
        .iter()
        .filter(|a| has_overlapping_interval(a))
        .count();

    println!("{:?}", count);
}
