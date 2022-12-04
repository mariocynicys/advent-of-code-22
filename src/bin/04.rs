#![allow(unused_variables,unused_imports)]

fn includes(r1: &(u32, u32), r2: &(u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}

fn overlaps(r1: &(u32, u32), r2: &(u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.0
}

fn common(input: &str, func: &dyn Fn(&(u32, u32), &(u32, u32)) -> bool) -> u32 {
    let mut sum = 0;
    let input = input.trim();
    for line in input.split('\n') {
        // Input format: id1-id2,id3-id4
        let ids: Vec<u32> = line.split(&['-', ','][..]).map(|n_str| n_str.parse().unwrap()).collect();
        let r1 = (ids[0], ids[1]);
        let r2 = (ids[2], ids[3]);
        // Note: checking r2 against r1 is useless in overlaps and will yeild the sa against checking r1 against r2.
        if func(&r1, &r2) || func(&r2, &r1) {
            sum += 1;
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(common(input, &includes))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(common(input, &overlaps))
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t4 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
