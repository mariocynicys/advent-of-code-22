#![allow(unused_variables, unused_imports)]

use std::ops::Deref;

struct Grid {
    /// Grid representing the solid & air points.
    grid: Vec<Vec<bool>>,
    /// Where the sand is pouring from.
    sand_source: (usize, usize),
}

impl Grid {
    fn new(coordinates: Vec<Vec<(usize, usize)>>, sand_source: (usize, usize)) -> Self {
        let max_x = *coordinates
            .iter()
            .flat_map(|l| l.iter().map(|(x, y)| x))
            .max()
            .unwrap();
        let max_y = *coordinates
            .iter()
            .flat_map(|l| l.iter().map(|(x, y)| y))
            .max()
            .unwrap();
        let mut grid = Self {
            grid: vec![vec![true; max_y + 1]; max_x + 1],
            sand_source,
        };
        coordinates
            .iter()
            .map(|l| {
                l.iter().reduce(|point1, point2| {
                    grid.draw_line(point1, point2);
                    point2
                });
            })
            .count();
        grid
    }

    /// Draws a line of rocks (solid point) into our grid.
    fn draw_line(&mut self, point1: &(usize, usize), point2: &(usize, usize)) {
        let (min_x, max_x) = (point1.0.min(point2.0), point1.0.max(point2.0));
        let (min_y, max_y) = (point1.1.min(point2.1), point1.1.max(point2.1));
        let (range_x, range_y) = (max_x - min_x, max_y - min_y);
        let xs = min_x..=max_x;
        let ys = min_y..=max_y;
        for (x, y) in xs
            .chain(vec![max_x; range_y])
            .zip(ys.chain(vec![max_y; range_x]))
        {
            self.grid[x][y] = false;
        }
    }

    fn in_grid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.grid.len() as i32 && y >= 0 && y < self.grid[0].len() as i32
    }

    fn blocked(&self, x: i32, y: i32) -> bool {
        // `self.grid` encodes an empty slot with true.
        self.in_grid(x, y) && !self.grid[x as usize][y as usize]
    }

    /// Creates a sand unit at the sand source and begins moving it till it's blocked (becomes solid).
    /// Returns whether the sand unit came to rest or on free fall.
    fn trace_sand(&mut self) -> bool {
        let (mut x, mut y) = (self.sand_source.0 as i32, self.sand_source.1 as i32);
        if self.blocked(x, y) {
            // For part2, if the source is blocked, can't produce.
            return false;
        }
        while self.in_grid(x, y) {
            // Try to go down.
            if !self.blocked(x, y + 1) {
                y += 1;
            }
            // Try to go down left.
            else if !self.blocked(x - 1, y + 1) {
                x -= 1;
                y += 1;
            }
            // Try to go down right.
            else if !self.blocked(x + 1, y + 1) {
                x += 1;
                y += 1;
            }
            // Sand came to rest, block it and return.
            else {
                self.grid[x as usize][y as usize] = false;
                return true;
            }
        }
        false
    }

    /// Converts a grid from part1 format to part2 format.
    /// Basically adds two more horizontal lines at the bottom and trims the grid horizontally if needed.
    fn convert(self) -> Self {
        // Add 2 extra horizontal lines, one filled with air & one filled with solid.
        let new_y_size = self.grid[0].len() + 2;
        // x coordinate of any falling sand can't exceed this width. You can deduce that from the pyramid shape.
        let new_x_size = new_y_size * 2 - 1;
        // Center the new sand source horizontally at the middle.
        let new_source = (new_x_size / 2, 0);
        // Translation values between the old and new grid.
        let (x_trans, y_trans) = (
            new_source.0 as i32 - self.sand_source.0 as i32,
            new_source.1 as i32 - self.sand_source.1 as i32,
        );
        let mut new_grid = Self {
            // Note that new_x_size might be less than the previous gird's s_size.
            grid: vec![vec![true; new_y_size]; new_x_size],
            sand_source: new_source,
        };
        // Copy the old grid into the new grid.
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                let (new_x, new_y) = (x as i32 + x_trans, y as i32 + y_trans);
                if new_grid.in_grid(new_x, new_y) {
                    new_grid.grid[new_x as usize][new_y as usize] = self.grid[x][y];
                }
            }
        }
        // Fill the very last line with solids.
        for x in 0..new_x_size {
            new_grid.grid[x][new_y_size - 1] = false;
        }
        new_grid
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.grid[0].len() {
            for x in 0..self.grid.len() {
                if self.sand_source == (x, y) {
                    print!("o")
                } else if self.grid[x][y] {
                    print!(".")
                } else {
                    print!("#")
                }
            }
            println!()
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let mut coordinates: Vec<Vec<(usize, usize)>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cor| {
                    let mut p = cor.split(',');
                    (
                        p.next().unwrap().parse().unwrap(),
                        p.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let min_x = coordinates
        .iter()
        .flat_map(|l| l.iter().map(|r| r.0))
        .min()
        .unwrap();
    let min_y = 0_usize; // We know sand source is always at (500, 0).

    let sand_source = (500 - min_x, 0 - min_y);
    // Update the x coordinate of all the starting and ending points.
    coordinates
        .iter_mut()
        .map(|l| l.iter_mut().map(|(x, y)| *x -= min_x).count())
        .count();

    Grid::new(coordinates, sand_source)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let mut rest_sand_count = 0;
    while grid.trace_sand() {
        rest_sand_count += 1;
    }
    Some(rest_sand_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input).convert();
    let mut rest_sand_count = 0;
    while grid.trace_sand() {
        //grid.print();
        rest_sand_count += 1;
    }
    Some(rest_sand_count)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t14 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
