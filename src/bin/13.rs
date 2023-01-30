#![allow(unused_variables, unused_imports)]

use std::{cmp::Ordering, fmt::Debug, ops::Deref};

enum Data {
    List(Vec<Data>),
    Number(u32),
}

impl Data {
    /// ever heard of python's eval, this function is blazingly faster.
    fn new(d: &str) -> Self {
        let d = d.trim();
        let d = &d[1..d.len() - 1];
        let mut this = vec![]; // self to be returned
        let mut nest_level = 0; // how many nesting are we in rn
        let mut index = 0; // an index on the string chars
        let mut next_list_start = 0; // the starting index of the next nested list (only 1 level nesting)
        for splt in d.split_inclusive(['[', ']', ',']) {
            if splt.starts_with('[') {
                if nest_level == 0 {
                    // record this list if it's a direct child
                    next_list_start = index;
                }
                nest_level += 1;
            } else if splt.ends_with(']') {
                nest_level -= 1;
                if nest_level == 0 {
                    // process this list if it's the end of a direct child
                    this.push(Data::new(&d[next_list_start..(index + splt.len())]))
                }
            } else if nest_level == 0 {
                // just a number, parse it.
                let s = splt.trim_end_matches(',');
                if !s.is_empty() {
                    this.push(Data::Number(s.parse().unwrap()));
                }
            }
            index += splt.len();
        }
        Self::List(this)
    }

    fn ord(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => n1.cmp(n2),
            (Self::List(l1), Self::List(l2)) => {
                for (i1, i2) in l1.deref().iter().zip(l2.deref().iter()) {
                    let order = i1.ord(i2);
                    if order != Ordering::Equal {
                        // don't continue if they are not equal
                        return order;
                    }
                }
                // break the tie based on the lists' length
                l1.deref().len().cmp(&l2.deref().len())
            }
            (Self::Number(n1), l2) => {
                let l1 = Data::List(vec![Self::Number(*n1)]);
                l1.ord(l2)
            }
            (l1, Self::Number(n2)) => {
                let l2 = Data::List(vec![Self::Number(*n2)]);
                l1.ord(&l2)
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Data, Data)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let pair: Vec<&str> = pair.lines().collect();
            (Data::new(pair[0]), Data::new(pair[1]))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let v = parse_input(input);
    Some(
        v.iter()
            .enumerate()
            .filter_map(|(i, (first, second))| {
                (first.ord(second) == Ordering::Less).then_some(i as u32 + 1)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let v = parse_input(input);
    let two = Data::new("[[2]]");
    let six = Data::new("[[6]]");
    let mut flat = vec![&two, &six];
    flat.extend(v.iter().flat_map(|(p1, p2)| vec![p1, p2]));
    flat.sort_by(|i1, i2| i1.ord(i2));

    let mut multiple = 1;
    for (index, item) in flat.iter().enumerate() {
        match (two.ord(item), six.ord(item)) {
            (Ordering::Equal, _) => multiple *= index as u32 + 1,
            (_, Ordering::Equal) => multiple *= index as u32 + 1,
            _ => (),
        }
    }
    Some(multiple)
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t13 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
