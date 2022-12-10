#![allow(unused_variables, unused_imports)]

enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(typ: &str) -> Self {
        match &typ[..4] {
            "noop" => Self::NoOp,
            "addx" => Self::AddX(typ[5..].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split('\n').map(Instruction::from).collect()
}

fn draw_output(lines: Vec<Vec<bool>>, chr: char, emp: char) -> String {
    lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|drawn| if *drawn { chr } else { emp })
                .chain(['\n'])
                .collect::<String>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_input(input);
    let mut x_reg = 1;
    let mut cycle = 0;
    let mut muls = 0;

    macro_rules! advance_cycle {
        // why did this have to be a macro? no reason :).
        ($cycle:expr, $x_reg:expr, $muls:expr) => {
            $cycle += 1;
            if $cycle % 40 == 20 {
                $muls += $cycle * $x_reg;
            }
        };
    }
    for instruction in instructions {
        advance_cycle!(cycle, x_reg, muls);
        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(v) => {
                advance_cycle!(cycle, x_reg, muls);
                x_reg += v;
            }
        }
    }
    Some(muls)
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = parse_input(input);
    let mut lines: Vec<Vec<bool>> = vec![vec![false; 40]; 6];
    let mut cycle = 0;
    let mut x_reg = 1;

    macro_rules! advance_cycle {
        ($cycle:expr, $x_reg:expr, $lines:expr) => {
            let (row, col) = ($cycle / 40, $cycle % 40);
            // If the 3-pixel sprite position covers the column the CRT currently scanning.
            if [$x_reg - 1, $x_reg, $x_reg + 1].contains(&(col as i32)) {
                // Light the pixel up.
                lines[row][col] = true;
            }
            $cycle += 1;
        };
    }
    for instruction in instructions {
        advance_cycle!(cycle, x_reg, lines);
        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(v) => {
                advance_cycle!(cycle, x_reg, lines);
                x_reg += v;
            }
        }
    }

    Some(draw_output(lines, '#', '.'))
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t10 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        let ans = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .to_string();
        println!("{}", ans);
        assert_eq!(part_two(&input), Some(ans));
    }
}
