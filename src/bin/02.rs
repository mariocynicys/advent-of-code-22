#![allow(unused_variables, unused_imports)]

#[derive(PartialEq, Eq, Debug, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Play {
    fn from(input: &str) -> Play {
        match input {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => panic!("Unknown mapping"),
        }
    }
}

impl Play {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score_against(&self, op: &Self) -> u32 {
        let round_score = match (self, op) {
            // 4. Loop back.
            (Self::Rock, Self::Scissors) => 6,
            (Self::Scissors, Self::Rock) => 0,
            // 1. Win.
            (x, y) if x.value() > y.value() => 6,
            // 2. Draw.
            (x, y) if x == y => 3,
            // 3. Lose.
            _ => 0,
        };
        round_score + self.value()
    }

    /// Returns the play needed to be played against the op (&self) to win the round.
    fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draw(&self) -> Self {
        self.clone()
    }

    fn lose(&self) -> Self {
        let mut possibilities = vec![Self::Rock, Self::Paper, Self::Scissors];
        possibilities.retain(|p| *p != self.win() && *p != self.draw());
        possibilities[0].clone()
    }

    fn score_for_outcome(outcome: &str, op: &Self) -> u32 {
        let play = match outcome {
            "X" => op.lose(),
            "Y" => op.draw(),
            "Z" => op.win(),
            _ => panic!("Unknown mapping"),
        };
        play.score_against(op)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tot_score = 0;
    for line in input.split('\n') {
        let plays: Vec<&str> = line.split_whitespace().collect();
        if plays.is_empty() {
            continue;
        };
        let elf = Play::from(plays[0].trim());
        let me = Play::from(plays[1].trim());
        tot_score += me.score_against(&elf);
    }
    Some(tot_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tot_score = 0;
    for line in input.split('\n') {
        let plays: Vec<&str> = line.split_whitespace().collect();
        if plays.is_empty() {
            continue;
        };
        let elf = Play::from(plays[0].trim());
        let outcome = plays[1].trim();
        tot_score += Play::score_for_outcome(outcome, &elf);
    }
    Some(tot_score)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t2 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
