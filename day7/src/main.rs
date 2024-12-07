use std::{
    fs,
    io::{BufRead, BufReader},
};

use anyhow::Result;

fn main() -> Result<()> {
    // let input_file = fs::File::open("puzzle_input_example.txt")?;
    let input_file = fs::File::open("puzzle_input.txt")?;
    let input_line = BufReader::new(input_file).lines();

    let mut total_calibration_number = 0;
    let mut total_calibration_number_with_concat = 0;
    for line in input_line {
        match line {
            Ok(equations_string) => {
                if let Some(colon_index) = equations_string.find(':') {
                    let (test_value_str, operands_str) = equations_string.split_at(colon_index);
                    let test_value = test_value_str.parse::<u64>()?;
                    let operands = operands_str
                        .split_whitespace()
                        .skip(1) // This is the ':'
                        .flat_map(|x| x.parse::<u64>())
                        .collect::<Vec<u64>>();
                    if equation_can_be_true(test_value, operands.clone()) {
                        total_calibration_number += test_value;
                    }
                    if equation_can_be_true_with_concat(test_value, operands) {
                        total_calibration_number_with_concat += test_value;
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    println!("Total Calibration Number: {}", total_calibration_number);
    println!(
        "Total Calibration Number with concat: {}",
        total_calibration_number_with_concat
    );

    Ok(())
}

fn equation_can_be_true(test_value: u64, operands: Vec<u64>) -> bool {
    let mut current_vals = Vec::new();
    for operand in operands {
        if current_vals.is_empty() {
            current_vals.push(operand);
        } else {
            current_vals = current_vals
                .iter()
                .flat_map(|x| vec![x + operand, x * operand])
                .collect::<Vec<u64>>();
        }
    }
    current_vals.contains(&test_value)
}

fn equation_can_be_true_with_concat(test_value: u64, operands: Vec<u64>) -> bool {
    let mut current_vals = Vec::new();
    for operand in operands {
        if current_vals.is_empty() {
            current_vals.push(operand);
        } else {
            current_vals = current_vals
                .iter()
                .flat_map(|x| {
                    vec![
                        x + operand,
                        x * operand,
                        (x.to_string() + operand.to_string().as_str())
                            .parse::<u64>()
                            .expect("Unable to parse after concatenation"),
                    ]
                })
                .collect::<Vec<u64>>();
        }
    }
    current_vals.contains(&test_value)
}
