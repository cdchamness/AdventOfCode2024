use anyhow::Result;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(input_file).lines();

    let mut reading_patterns = true;
    let mut towel_patterns = Vec::new();
    let mut display_patterns = Vec::new();

    // Read Input
    for line in input_lines {
        match line {
            Ok(line_string) => {
                if reading_patterns {
                    towel_patterns = line_string
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>();
                    reading_patterns = false;
                } else if !line_string.is_empty() {
                    display_patterns.push(line_string.clone());
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }

    // Part 1
    let mut possible_displays = 0;
    for display_pattern in &display_patterns {
        let mut full_check_list = Vec::new();
        let mut found_soluton = false;
        let mut queue = vec![display_pattern.clone()];
        while let Some(display) = queue.pop() {
            for towel in &towel_patterns {
                if display.starts_with(towel) {
                    let x = display.chars().skip(towel.len()).collect::<String>();
                    if x.is_empty() {
                        possible_displays += 1;
                        queue.clear();
                        found_soluton = true;
                    } else if !full_check_list.contains(&x) && !queue.contains(&x) && !found_soluton
                    {
                        queue.push(x.clone());
                        full_check_list.push(x);
                    }
                }
            }
        }
    }
    println!("Possible arrangements: {}", possible_displays);

    // Part 2
    let mut unique_ways = 0;
    let mut previous_solutions: HashMap<String, usize> = HashMap::new();
    for display_pattern in display_patterns {
        unique_ways += count_ways(display_pattern, &towel_patterns, &mut previous_solutions);
    }
    println!("Unique arrangements: {}", unique_ways);

    Ok(())
}

fn count_ways(
    display_pattern: String,
    towel_patterns: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(val) = cache.get(&display_pattern) {
        *val
    } else {
        let mut total = 0;
        for towel in towel_patterns {
            if display_pattern.starts_with(towel) {
                let x = display_pattern
                    .chars()
                    .skip(towel.len())
                    .collect::<String>();
                if x.is_empty() {
                    total += 1;
                } else {
                    total += count_ways(x, towel_patterns, cache);
                }
            }
        }
        cache.insert(display_pattern, total);
        total
    }
}
