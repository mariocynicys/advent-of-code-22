#![allow(unused_variables, unused_imports)]

use std::{
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, BTreeSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Point(usize, usize);
type Grid = Vec<Vec<u32>>;

trait GridUtils {
    fn new(rows: usize, cols: usize) -> Self;
    fn successors(&self, p: &Point) -> Vec<Option<Point>>;
    fn value(&self, p: &Point) -> u32;
}

impl GridUtils for Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        vec![vec![0; cols]; rows]
    }

    fn successors(&self, p: &Point) -> Vec<Option<Point>> {
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let rows = self.len() as i32;
        let cols = self[0].len() as i32;
        let px = p.0 as i32;
        let py = p.1 as i32;
        directions
            .iter()
            .map(|(i, j)| {
                if (0_i32..rows).contains(&(px + i)) && (0_i32..cols).contains(&(py + j)) {
                    Some(Point((px + i) as usize, (py + j) as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    fn value(&self, p: &Point) -> u32 {
        self[p.0][p.1]
    }
}

fn parse_input(input: &str) -> (Grid, Point, Point, Vec<Point>) {
    let input = input.trim();
    let rows = input.lines();
    let mut start = None;
    let mut end = None;
    let mut lowest_elevation = Vec::new(); // For part 2.
    let result: Grid = rows
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, mut c)| {
                    if c == 'S' {
                        start = Some(Point(i, j));
                        c = 'a';
                    }
                    if c == 'E' {
                        end = Some(Point(i, j));
                        c = 'z';
                    }
                    if c == 'a' {
                        // For part 2.
                        lowest_elevation.push(Point(i, j));
                    }
                    c.try_into().unwrap()
                })
                .collect()
        })
        .collect();
    (result, start.unwrap(), end.unwrap(), lowest_elevation)
}

fn search(grid: &Grid, start: Point, end: Point) -> Option<u32> {
    let mut explored = <Grid as GridUtils>::new(grid.len(), grid[0].len());
    let mut frontier = BinaryHeap::new();
    frontier.push((Reverse(0), start));

    while let Some((Reverse(cost), point)) = frontier.pop() {
        if point == end {
            return Some(cost);
        }
        if explored.value(&point) == 1 {
            continue;
        }
        explored[point.0][point.1] = 1;
        let next_points = grid
            .successors(&point)
            .into_iter()
            .filter(|o| o.is_some() && grid.value(&point) + 1 >= grid.value(o.as_ref().unwrap()))
            .map(|o| (Reverse(cost + 1), o.unwrap()));
        frontier.extend(next_points);
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end, _) = parse_input(input);
    search(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, end, lowest_elevations) = parse_input(input);
    lowest_elevations
        .iter()
        // This is running BFS for every possible starting point.
        // Think this could be better performed with DP.
        .filter_map(|starting_point| search(&grid, starting_point.clone(), end.clone()))
        .min()
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t12 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
