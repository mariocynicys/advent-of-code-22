#![allow(unused_variables, unused_imports)]

use std::collections::HashSet;

const V: usize = 0; // U & D
const H: usize = 1; // R & L

fn parse_input(input: &str) -> Vec<(usize, i32)> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = match parts[0] {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (1, 0),
                "D" => (-1, 0),
                _ => unreachable!(),
            };
            let amount = parts[1].parse::<i32>().unwrap() * (direction.0 + direction.1);
            ((direction.0 == 0) as usize, amount)
        })
        .collect()
}

fn new_heads(head: [i32; 2], motion: (usize, i32)) -> Vec<[i32; 2]> {
    (1..=motion.1.abs())
        .map(|i| {
            let mut head = head;
            head[motion.0] += i * motion.1.signum();
            head
        })
        .collect()
}

fn new_tail(head: [i32; 2], tail: [i32; 2]) -> [i32; 2] {
    match (head[V] - tail[V], head[H] - tail[H]) {
        // Shouldn't move. Inside the 9-cell square.
        (0 | 1 | -1, 0 | 1 | -1) => tail,
        // Should move vertically. Up or down by 2.
        (v, 0) if [2, -2].contains(&v) => [tail[V] + v.signum(), tail[H]],
        // Should move horizontally. Left or right by 2.
        (0, h) if [2, -2].contains(&h) => [tail[V], tail[H] + h.signum()],
        // Should move diagonally.
        (v, h) => [tail[V] + v.signum(), tail[H] + h.signum()],
    }
}

fn common(input: &str, chain_length: usize) -> usize {
    let motions = parse_input(input);
    let mut tail_set: HashSet<[i32; 2]> = HashSet::new();
    // chain[0] == head, chain[-1] == tail
    let mut chain = vec![[0, 0]; chain_length];

    for motion in motions {
        for new_head in new_heads(chain[0], motion) {
            // Update the head;
            chain[0] = new_head;
            // Update the knots.
            let mut last_knot = new_head;
            for knot in chain.iter_mut() {
                *knot = new_tail(last_knot, *knot);
                last_knot = *knot;
            }
            // Record the tail position.
            tail_set.insert(chain[chain.len() - 1]);
        }
    }

    tail_set.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    // This function body can be replaced with the following line:
    // return Some(common(input, 2));
    let motions = parse_input(input);
    let mut tail_set: HashSet<[i32; 2]> = HashSet::new();
    let mut head = [0, 0];
    let mut tail = [0, 0];

    for motion in motions {
        for new_head in new_heads(head, motion) {
            // Update the head.
            head = new_head;
            // Update the tail.
            tail = new_tail(head, tail);
            // Record the tail position.
            tail_set.insert(tail);
        }
    }

    Some(tail_set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(common(input, 10))
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t9 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
