use anyhow::Result;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<()> {
    // Read Input
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let mut stones_string = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut stones_string)?;
    let mut stone_values = stones_string
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<u64>>();

    // !!!!!!!!!!!!!!!!!!!! OLD VERSION !!!!!!!!!!!!!!!!!!!!
    // this version is slower, but keeping it here because it was the way I
    // solved the 1st problem, using the HashMap version is much faster

    // // Find stone count after 25 blinks
    // let mut updated_values = Vec::new();
    // for _ in 0..25 {
    //     // Loop over each stone, update, and store the results in temp vec
    //     for stone in stone_values {
    //         let new_vals = update_stone_value(stone);
    //         for val in new_vals {
    //             updated_values.push(val);
    //         }
    //     }
    //     // store updated values, clear temp vec
    //     stone_values = updated_values.clone();
    //     updated_values.clear();
    // }
    // println!("Number of Stones after 25 blinks: {}", stone_values.len());
    // !!!!!!!!!!!!!!!!!!!! OLD VERSION !!!!!!!!!!!!!!!!!!!!

    // Find stone count after 25 and 75 blinks
    // Reinitalize
    let stone_values = stones_string
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<u64>())
        .collect::<Vec<u64>>();

    // First it is clear that brute forcing to 75 blinks is not a viable option
    // Then, notice that the order of the stones does not matter
    // we actually only need the set of numbers and to keep track of how many there are of each value.
    // So, we do the same as before, but instead use a HashMap between the stones
    // values and how many stone have that particular value instead of a Vec of stone values

    // Initialize HashMap
    let mut stone_hash: HashMap<u64, usize> = HashMap::new();
    for stone in stone_values {
        add_stone_to_hashmap(&mut stone_hash, stone, 1);
    }

    // Duplicate procedure from above, but with a HashMap instead
    let mut updated_hash: HashMap<u64, usize> = HashMap::new();
    for blinks in 1..=75 {
        for (stone_val, stone_count) in stone_hash.iter() {
            for new_val in update_stone_value(*stone_val) {
                // update temp HashMap for each new val
                add_stone_to_hashmap(&mut updated_hash, new_val, *stone_count);
            }
        }
        // store updated values, reset temp hash
        stone_hash = updated_hash.clone();
        updated_hash.drain();
        // print result at 25 blinks
        if blinks == 25 {
            let total_stone_count: usize = stone_hash.values().sum();
            println!("Number of Stones after 25 blinks: {}", total_stone_count);
        }
    }
    let total_stone_count: usize = stone_hash.values().sum();
    println!("Number of Stones after 75 blinks: {}", total_stone_count);

    Ok(())
}

fn update_stone_value(current_value: u64) -> Vec<u64> {
    if current_value == 0 {
        return vec![1];
    }
    let digit_string = current_value.to_string();
    let digit_count = digit_string.len();
    if digit_count % 2 == 0 {
        let (left, right) = digit_string.split_at(digit_count / 2);
        let left_digit = left.parse::<u64>().expect("Unable to parse left digit");
        let right_digit = right.parse::<u64>().expect("Unable to parse right digit");
        return vec![left_digit, right_digit];
    }
    vec![current_value * 2024]
}

fn add_stone_to_hashmap(hash_map: &mut HashMap<u64, usize>, stone: u64, count: usize) {
    if hash_map.contains_key(&stone) {
        let current_count = hash_map
            .get(&stone)
            .expect("Unable to get value from key in hash");
        hash_map.insert(stone, current_count + count);
    } else {
        hash_map.insert(stone, count);
    }
}
