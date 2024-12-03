use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_line = BufReader::new(input_file).lines();

    let mut safe_count: u64 = 0;
    let mut dampener_safe_count: u64 = 0;
    for line in input_line {
        match line {
            Ok(levels_string) => {
                let levels = parse_line(levels_string)?;
                if is_safe(&levels) {
                    safe_count += 1;
                    dampener_safe_count += 1
                } else if is_safe_with_dampener(&levels) {
                    dampener_safe_count += 1;
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    println!("Number of Safe Reports: {}", safe_count);
    println!("Number of Dampener Safe Reports: {}", dampener_safe_count);

    Ok(())
}

fn parse_line(levels_string: String) -> Result<Vec<i64>> {
    let mut levels = Vec::new();
    for str_val in levels_string.split_whitespace() {
        if let Ok(val) = str_val.parse::<i64>() {
            levels.push(val);
        } else {
            return Err(anyhow::Error::msg("Failed to parse str to u64"));
        }
    }
    Ok(levels)
}

fn is_safe(levels: &[i64]) -> bool {
    let mut diffs = Vec::new();
    for i in 0..(levels.len() - 1) {
        diffs.push(levels[i + 1] - levels[i]);
    }
    // Now that I have the differences between levels, I can just check for the fail cases and return early
    // if none of the fail cases are satisfied it is safe
    if diffs.contains(&0) {
        // No change
        return false;
    }
    if diffs.iter().min().is_some_and(|x| x.is_negative())
        && diffs.iter().max().is_some_and(|x| x.is_positive())
    {
        // Increase and decrease
        return false;
    }
    if !diffs
        .into_iter()
        .filter(|x| x.abs() > 3)
        .collect::<Vec<i64>>()
        .is_empty()
    {
        // There was at least 1 change that was larger than 3
        return false;
    }
    true
}

fn is_safe_with_dampener(levels: &[i64]) -> bool {
    let mut dampener_safe = Vec::new();
    for i in 0..levels.len() {
        let mut dampened_levels = levels.to_vec();
        dampened_levels.remove(i);
        dampener_safe.push(is_safe(&dampened_levels));
    }

    dampener_safe.contains(&true)
}
