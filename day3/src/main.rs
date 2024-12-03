use anyhow::Result;
use regex::Regex;

use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<()> {
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let mut full_instructions = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut full_instructions)?;

    // Values to store running totals
    let mut total = 0;
    let mut conditional_total = 0;

    // Regexes
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don\'t\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // Analysis
    // Part 1
    let regex_match = mul_regex.find_iter(full_instructions.as_str());
    for mul_match in regex_match {
        if let Some((a, b)) = mul_match.as_str().split_once(',') {
            let a_clean = a.trim_start_matches("mul(").parse::<u64>()?;
            let b_clean = b.trim_end_matches(")").parse::<u64>()?;
            total += a_clean * b_clean;
        }
    }

    // Part 2 - only run mul()s between a do() and a don't()
    let do_match_split = do_regex.split(full_instructions.as_str());
    // each section starts right after a do()
    for section in do_match_split {
        // grab everything until we find a don't()
        let instructions_to_run = dont_regex.split(section).take(1).collect::<String>();

        // now do the mul()s only on the parts that come after a do() but before a don't()
        let regex_match = mul_regex.find_iter(instructions_to_run.as_str());
        for mul_match in regex_match {
            if let Some((a, b)) = mul_match.as_str().split_once(',') {
                let a_clean = a.trim_start_matches("mul(").parse::<u64>()?;
                let b_clean = b.trim_end_matches(")").parse::<u64>()?;
                conditional_total += a_clean * b_clean;
            }
        }
    }

    println!("Total: {}", total);
    println!("Conditional Total: {}", conditional_total);
    Ok(())
}
