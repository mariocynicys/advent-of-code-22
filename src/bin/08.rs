#![allow(unused_variables, unused_imports)]
#![allow(clippy::comparison_chain, clippy::needless_range_loop)]

fn input_to_grid(input: &str) -> Vec<Vec<i32>> {
    let input = input.trim();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut grid = vec![vec![0; lines[0].len()]; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.as_bytes().iter().enumerate() {
            grid[i][j] = *ch as i32;
        }
    }
    grid
}

fn get_score(grid: &Vec<Vec<i32>>, (i, j): (usize, usize)) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let (mut up, mut down, mut left, mut right) = (i, rows - 1 - i, j, cols - 1 - j);
    for ii in 0..rows {
        if grid[ii][j] >= grid[i][j] {
            if ii < i {
                up = i - ii;
            } else if ii > i {
                down = ii - i;
                break;
            }
        }
    }
    for jj in 0..cols {
        if grid[i][jj] >= grid[i][j] {
            if jj < j {
                left = j - jj;
            } else if jj > j {
                right = jj - j;
                break;
            }
        }
    }
    (up * down * left * right) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input_to_grid(input);
    let mut vis = vec![vec![0; grid[0].len()]; grid.len()];
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        let mut max_l = -1;
        let mut max_r = -1;
        for j_l in 0..cols {
            let j_r = cols - j_l - 1;

            if max_l < grid[i][j_l] {
                vis[i][j_l] = 1;
                max_l = grid[i][j_l];
            }

            if max_r < grid[i][j_r] {
                vis[i][j_r] = 1;
                max_r = grid[i][j_r];
            }
        }
    }

    for j in 0..cols {
        let mut max_d = -1;
        let mut max_u = -1;
        for i_d in 0..rows {
            let i_u = rows - i_d - 1;

            if max_d < grid[i_d][j] {
                vis[i_d][j] = 1;
                max_d = grid[i_d][j];
            }

            if max_u < grid[i_u][j] {
                vis[i_u][j] = 1;
                max_u = grid[i_u][j];
            }
        }
    }

    Some(vis.iter().map(|r| r.iter().sum::<u32>()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input_to_grid(input);
    let mut score = vec![vec![0; grid[0].len()]; grid.len()];
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 1..rows {
        for j in 1..cols {
            score[i][j] = get_score(&grid, (i, j));
        }
    }

    score.iter().map(|r| r.iter().max().unwrap()).max().copied()
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t8 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
