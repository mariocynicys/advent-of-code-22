#![allow(unused_variables, unused_imports)]

use std::{cell::RefCell, collections::VecDeque};

struct Monkey {
    // Items it currently holds.
    items: RefCell<VecDeque<u64>>,
    // The operation used to update the worry level.
    operation: Box<dyn Fn(u64) -> u64>,
    // Test to determine to which monkey to pass the items.
    test: Box<dyn Fn(u64) -> usize>,
    // The number of inspections it has performed.
    inspections: RefCell<u64>,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        test: Box<dyn Fn(u64) -> usize>,
    ) -> Self {
        Self {
            items: RefCell::new(items),
            operation,
            test,
            inspections: RefCell::new(0),
        }
    }

    fn pick(&self) -> VecDeque<u64> {
        *self.inspections.borrow_mut() += self.items.borrow().len() as u64;
        self.items.borrow_mut().drain(..).collect()
    }

    fn new_worry_level(&self, old: u64) -> u64 {
        (self.operation)(old)
    }

    fn get_throw_to(&self, worry_level: u64) -> usize {
        (self.test)(worry_level)
    }

    fn receive(&self, item: u64) {
        self.items.borrow_mut().push_back(item)
    }
}

fn op_str_to_fn(op: &str) -> Box<dyn Fn(u64) -> u64> {
    let op: Vec<&str> = op
        .trim_start_matches("Operation: new = ")
        .split(' ')
        .collect();
    let operand1 = op[0].to_string();
    let operation = op[1].to_string();
    let operand2 = op[2].to_string();

    Box::new(move |old| {
        let op1 = if operand1 == "old" {
            old
        } else {
            operand1.parse().unwrap()
        };
        let op2 = if operand2 == "old" {
            old
        } else {
            operand2.parse().unwrap()
        };
        match operation.as_str() {
            "+" => op1 + op2,
            "*" => op1 * op2,
            _ => unimplemented!("Dunno this operation: {}", operation),
        }
    })
}

fn test_str_to_fn(test: &[&str]) -> Box<dyn Fn(u64) -> usize> {
    let monkey1 = test[1]
        .trim_start_matches("If true: throw to monkey ")
        .parse()
        .unwrap();
    let monkey2 = test[2]
        .trim_start_matches("If false: throw to monkey ")
        .parse()
        .unwrap();
    if test[0].starts_with("Test: divisible by ") {
        let div: u64 = test[0]
            .trim_start_matches("Test: divisible by ")
            .parse()
            .unwrap();
        Box::new(move |n| if n % div == 0 { monkey1 } else { monkey2 })
    } else {
        unimplemented!("Dunno this test: {}", test[0]);
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .trim()
        .split("\n\n")
        .map(|monkey| {
            let lines = monkey.lines().map(|l| l.trim()).collect::<Vec<_>>();
            let items = lines[1]
                .trim_start_matches("Starting items: ")
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect();
            let operation = op_str_to_fn(lines[2]);
            let test = test_str_to_fn(&lines[3..=5]);
            Monkey::new(items, operation, test)
        })
        .collect()
}

pub fn common(input: &str, rounds: u64, div_: u64, mod_: u64) -> Option<u64> {
    let mut monkeys = parse_input(input);
    for round in 0..rounds {
        for monkey in monkeys.iter() {
            for item in monkey.pick() {
                let worry_level = monkey.new_worry_level(item) % mod_ / div_;
                let send_to = monkey.get_throw_to(worry_level);
                monkeys[send_to].receive(worry_level);
            }
        }
    }
    monkeys.sort_by(|m1, m2| m2.inspections.borrow().cmp(&m1.inspections.borrow()));
    let first_most_inspections = *monkeys[0].inspections.borrow();
    let second_most_inspections = *monkeys[1].inspections.borrow();
    Some(first_most_inspections * second_most_inspections)
}

pub fn part_one(input: &str) -> Option<u64> {
    common(input, 20, 3, u64::MAX)
}

pub fn part_two(input: &str) -> Option<u64> {
    let all_divisors: u64 = input
        .trim()
        .split("divisible by ")
        .skip(1)
        .map(|s| s.lines().next().unwrap().parse::<u64>().unwrap())
        .product();

    common(input, 10_000, 1, all_divisors)
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t11 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
