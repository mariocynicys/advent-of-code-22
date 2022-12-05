#![allow(unused_variables, unused_imports)]

type CrateMap = Vec<Vec<u8>>;
type Instructions = Vec<(usize, usize, usize)>;
fn parse(input: &str) -> (CrateMap, Instructions) {
    // Be careful not to trim the start so not to change how the input map looks like.
    let input = input.trim_end();
    // Split the input into a map and instructions.
    let mut splitter = input.split("\n\n");

    // 1- Parse the map.
    let map = splitter.next().unwrap();
    // Work from the bottom up.
    let mut map = map.split('\n').rev();
    // The first line (last line when up bottom) will tell the number of stacks.
    let n_stacks: usize = map
        .next()
        .unwrap()
        .split_whitespace()
        // Reverse, get the first element (last from reved), and parse it. That's the number of stacks.
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    // Init a crate map with the capacity of `n_stacks`.
    let mut crate_map: CrateMap = vec![Vec::new(); n_stacks];
    for line in map {
        // From each 4 characters, get the second which represents the letter if there is any
        // or a space if no cargo in this position.
        let crates = line.as_bytes().chunks(4).map(|cs| cs[1]);
        for (i, one_crate) in crates.enumerate() {
            // Skip white spaces.
            if one_crate != " ".as_bytes()[0] {
                crate_map[i].push(one_crate);
            }
        }
    }

    // 2- Parse the instructions.
    let instructions = splitter
        .next()
        .unwrap()
        .split('\n')
        .map(|s| {
            let v: Vec<&str> = s.split_whitespace().collect();
            let amount = v[1].parse().unwrap();
            let from: usize = v[3].parse().unwrap();
            let to: usize = v[5].parse().unwrap();
            // -1 to make from & to 0-indexed.
            (amount, from - 1, to - 1)
        })
        .collect();

    (crate_map, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut m, i) = parse(input);
    // Apply the instructions.
    for (amount, from, to) in i {
        // Move crates one by one.
        for i in 0..amount {
            let cargo = m[from].pop().unwrap();
            m[to].push(cargo);
        }
    }

    // Skim the top crates.
    let s = m
        .iter()
        .filter_map(|stk| stk.last())
        .map(|&c| c as char)
        .collect();

    Some(s)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut m, i) = parse(input);
    // Apply the instructions.
    for (amount, from, to) in i {
        // Move the whole amount in one go.
        let start_loading_from = m[from].len() - amount;
        let cargos_to_move: Vec<u8> = m[from].drain(start_loading_from..).collect();
        m[to].extend(cargos_to_move);
    }

    // Skim the top crates.
    let s = m
        .iter()
        .filter_map(|stk| stk.last())
        .map(|&c| c as char)
        .collect();

    Some(s)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t5 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
