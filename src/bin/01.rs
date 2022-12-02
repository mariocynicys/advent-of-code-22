#![allow(unused_variables,unused_imports)]
use contest_algorithms;

fn common(input: &str) -> Vec<u32> {
    let mut elf_cal = vec![0];
    for line in input.split("\n") {
        let line = line.trim();
        if line == "" {
            elf_cal.push(0);
        } else {
            *elf_cal.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
    }
    elf_cal
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_cal = common(input);
    Some(*elf_cal.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_cal = common(input);
    let mut tot_cal = 0;
    elf_cal.sort();
    elf_cal.reverse();
    for cal in &elf_cal[..3] {
        tot_cal += cal;
    }
    Some(tot_cal)
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
