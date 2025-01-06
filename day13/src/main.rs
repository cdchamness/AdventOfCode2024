use anyhow::Result;
use regex::Regex;

use std::{
    fs,
    io::{self, BufRead},
    ops::{Add, Mul},
};

const CONVERSION_OFFSET: Position = Position {
    x: 10000000000000,
    y: 10000000000000,
};

fn main() -> Result<()> {
    // let input_file = fs::File::open("puzzle_input_example.txt")?;
    let input_file = fs::File::open("puzzle_input.txt")?;
    let input_lines: Vec<String> = io::BufReader::new(input_file)
        .lines()
        .map_while(Result::ok)
        .collect();
    let claw_machine_lines = input_lines.chunks(4);

    let mut claw_machines = Vec::new();
    for claw_machine_str in claw_machine_lines {
        claw_machines.push(ClawMachine::from_slice_of_string(claw_machine_str));
    }

    let total_min_cost = claw_machines
        .iter()
        .fold(0, |acc, claw| acc + claw.find_min_cost_solution());

    println!("Total Min Cost: {}", total_min_cost);

    let total_corrected_min_cost = claw_machines
        .iter()
        .fold(0, |acc, claw| acc + claw.find_min_cost_with_offset());

    println!("Total Min Cost: {}", total_corrected_min_cost);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position { x, y }
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<Position> for i64 {
    type Output = Position;
    fn mul(self, rhs: Position) -> Position {
        Position {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Position,
    button_b: Position,
    target: Position,
}

impl ClawMachine {
    fn from_slice_of_string(input: &[String]) -> ClawMachine {
        let button_a = parse_digits(input.first().expect("input missing 1st value"));
        let button_b = parse_digits(input.get(1).expect("input missing 2nd value"));
        let target = parse_digits(input.get(2).expect("input missing 3rd value"));

        ClawMachine {
            button_a,
            button_b,
            target,
        }
    }

    fn find_min_cost_solution(&self) -> i64 {
        let mut min_cost = i64::MAX;
        // with a max press count of 100 we can easily just try all
        // 10,000 combos per claw machine and find the cheapest one that works
        for a_presses in 0..=100 {
            for b_presses in 0..=100 {
                let current_cost = a_presses * 3 + b_presses;
                if current_cost < min_cost
                    && a_presses * self.button_a + b_presses * self.button_b == self.target
                {
                    min_cost = current_cost
                }
            }
        }
        if min_cost != i64::MAX {
            min_cost
        } else {
            0
        }
    }

    fn find_min_cost_with_offset(&self) -> i64 {
        // it is clear that we cannot brute force try every combo to calculate the solution
        // however we reframe the problem with Linear Algebra
        //                M                 *       x         =       b
        // _______________________________     _____________     ____________
        // | button_a_x   ,   button_b_x |  *  | a_presses |  =  | target_x |
        // | button_a_y   ,   button_b_y |     | b_presses |     | target_y |
        // -------------------------------     -------------     ------------
        //  This implies then that we may find x by inverting M as
        //      x         =                    M^-1                     *       b
        // _____________       1     _________________________________     ____________
        // | a_presses |  =   ---  * |  button_b_y   ,   -button_b_x |  *  | target_x |
        // | b_presses |     det(M)  | -button_a_y   ,    button_a_x |     | target_y |
        // -------------             ---------------------------------     ------------

        let corrected_target = self.target + CONVERSION_OFFSET;
        let det_m = self.button_a.x * self.button_b.y - self.button_b.x * self.button_a.y;
        let a_presses =
            (self.button_b.y * corrected_target.x - self.button_b.x * corrected_target.y) / det_m;
        let b_presses =
            (self.button_a.x * corrected_target.y - self.button_a.y * corrected_target.x) / det_m;

        // this is to check if it is actually a solution in the Natural Numbers
        let test_position = a_presses * self.button_a + b_presses * self.button_b;
        if test_position == corrected_target {
            3 * a_presses + b_presses
        } else {
            0
        }
    }
}

fn parse_digits(button_string: &str) -> Position {
    let digit_regex = Regex::new(r"\d+").unwrap();
    let digits = digit_regex
        .find_iter(button_string)
        .flat_map(|s| s.as_str().parse::<i64>())
        .collect::<Vec<i64>>();
    Position::new(digits[0], digits[1])
}
