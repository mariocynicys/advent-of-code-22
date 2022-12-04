#![allow(unused_variables,unused_imports)]
use std::collections::HashSet;

fn ascii_to_priority(ascii: u8) -> u8 {
    // If upper case.
    if (0x41..=0x5a).contains(&ascii) {
        ascii - 0x41 + 27
    } else {
        ascii - 0x61 + 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    // Remove trailing '\n's.
    let input = input.trim_end();
    for line in input.split('\n') {
        // Skip the first split element as it will be an empty string.
        let mut chars: Vec<String> = line.trim().split("").map(|s| s.to_string()).skip(1).collect();
        // Skip the last element as it will be an empty string.
        chars.pop();
        // Get the first and second compartment.
        let mid = chars.len() / 2;
        let comp1: HashSet<String> = HashSet::from_iter(chars[..mid].iter().cloned());
        let comp2: HashSet<String> = HashSet::from_iter(chars[mid..].iter().cloned());
        // Get the common char in the two compartments. Note that the intersection should yeild only one char.
        let common = comp1.intersection(&comp2).next().unwrap().as_bytes()[0];
        sum += ascii_to_priority(common) as u32;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let team_size = 3;
    let mut sum: u32 = 0;
    // Remove trailing '\n's.
    let input = input.trim_end();
    for lines in input.split('\n').collect::<Vec<&str>>().chunks(team_size) {
        let mut intersec: Option<HashSet<String>> = None;
        for line in lines {
            // Skip the first split element as it will be an empty string.
            let mut chars: Vec<String> = line.trim().split("").map(|s| s.to_string()).skip(1).collect();
            // Skip the last element as it will be an empty string.
            chars.pop();
            let chars_set: HashSet<String> = HashSet::from_iter(chars);
            intersec = if let Some(intersec) = intersec {
                Some(intersec.intersection(&chars_set).cloned().collect())
            } else {
                Some(chars_set)
            }
        }
        // Get the common char in the two compartments. Note that the intersection should yeild only one char.
        let common = intersec.unwrap().iter().next().unwrap().as_bytes()[0];
        sum += ascii_to_priority(common) as u32;
    }
    Some(sum)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t3 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
