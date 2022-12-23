use std::collections::HashMap;

#[derive(PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn get_desired_choice(&self, desired_result: &RoundResult) -> Choice {
        match *self {
            Choice::Rock => match *desired_result {
                RoundResult::Win => Choice::Paper,
                RoundResult::Lose => Choice::Scissors,
                RoundResult::Draw => Choice::Rock,
            },
            Choice::Paper => match *desired_result {
                RoundResult::Win => Choice::Scissors,
                RoundResult::Lose => Choice::Rock,
                RoundResult::Draw => Choice::Paper,
            },
            Choice::Scissors => match *desired_result {
                RoundResult::Win => Choice::Rock,
                RoundResult::Lose => Choice::Paper,
                RoundResult::Draw => Choice::Scissors,
            },
        }
    }

    fn get_choice_score(&self) -> isize {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_result_score(&self, desired_result: &RoundResult) -> isize {
        match *desired_result {
            RoundResult::Win => 6,
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
        }
    }
}

fn main() {
    let strategy_lines = include_str!("../inputs/day2.txt");

    let opponent_lookup: HashMap<&str, Choice> = HashMap::from([
        ("A", Choice::Rock),
        ("B", Choice::Paper),
        ("C", Choice::Scissors),
    ]);

    let result_lookup: HashMap<&str, RoundResult> = HashMap::from([
        ("X", RoundResult::Lose),
        ("Y", RoundResult::Draw),
        ("Z", RoundResult::Win),
    ]);

    let rounds: Vec<Vec<&str>> = strategy_lines
        .lines()
        .map(|l| return l.split(" ").collect())
        .collect();

    let score: isize = rounds
        .iter()
        .map(|round| {
            let opp_choice = opponent_lookup.get(round[0]).unwrap();
            let desired_result = result_lookup.get(round[1]).unwrap();

            let player_choice = opp_choice.get_desired_choice(desired_result);

            return player_choice.get_choice_score()
                + player_choice.get_result_score(desired_result);
        })
        .sum();

    println!("{:?}", score)
}
