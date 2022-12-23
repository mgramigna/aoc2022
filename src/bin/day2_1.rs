use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn wins_against(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn get_choice_score(&self) -> isize {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_round_score(&self, opponent_choice: &Choice) -> isize {
        let choice_score = self.get_choice_score();
        if *self == *opponent_choice {
            return choice_score + 3;
        }

        if self.wins_against() == *opponent_choice {
            return choice_score + 6;
        }

        return choice_score;
    }
}

fn main() {
    let strategy_lines = include_str!("../inputs/day2.txt");

    let opponent_lookup: HashMap<&str, Choice> = HashMap::from([
        ("A", Choice::Rock),
        ("B", Choice::Paper),
        ("C", Choice::Scissors),
    ]);

    let player_lookup: HashMap<&str, Choice> = HashMap::from([
        ("X", Choice::Rock),
        ("Y", Choice::Paper),
        ("Z", Choice::Scissors),
    ]);

    let rounds: Vec<Vec<&str>> = strategy_lines
        .lines()
        .map(|l| return l.split(" ").collect())
        .collect();

    let total_score: isize = rounds
        .iter()
        .map(|round| {
            let opp_choice = opponent_lookup.get(round[0]).unwrap();
            let player_choice = player_lookup.get(round[1]).unwrap();

            return player_choice.get_round_score(opp_choice);
        })
        .sum();

    println!("{:?}", total_score)
}
