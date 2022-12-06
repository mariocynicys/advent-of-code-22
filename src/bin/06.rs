#![allow(unused_variables, unused_imports)]
use std::collections::{HashSet, VecDeque};

fn common(input: &str, marker_size: usize) -> Option<u32> {
    let input = Vec::from(input.trim());
    let m: HashSet<u32>;
    let mut deq = VecDeque::from_iter(&input[..marker_size]);
    for (i, new_letter) in input.iter().enumerate().skip(marker_size) {
        deq.pop_front();
        deq.push_back(new_letter);
        if HashSet::<&&u8>::from_iter(deq.iter()).len() == marker_size {
            return Some(i as u32 + 1);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    common(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    common(input, 14)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t6 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
