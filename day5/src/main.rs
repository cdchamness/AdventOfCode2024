use anyhow::Result;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(input_file).lines();

    // Read Input
    let mut order_rules = Vec::new();
    let mut update_orders = Vec::new();
    for line in input_lines {
        match line {
            Ok(line_string) => {
                if line_string.contains("|") {
                    if let Some((before, after)) = line_string.split_once('|') {
                        let before_num = before.parse::<u32>()?;
                        let after_num = after.parse::<u32>()?;
                        order_rules.push([before_num, after_num]);
                    }
                } else if !line_string.is_empty() {
                    update_orders.push(
                        line_string
                            .split(",")
                            .flat_map(|s| s.parse::<u32>())
                            .collect::<Vec<u32>>(),
                    );
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }

    // Calculate Sum of Middle Indexes
    let mut correct_middle_total = 0;
    let mut incorrect_middle_total = 0;
    for update_order in update_orders {
        if is_good_order(&update_order, &order_rules) {
            correct_middle_total += update_order[update_order.len() / 2];
        } else {
            let new_order = fix_update_order(&update_order, &order_rules);
            incorrect_middle_total += new_order[new_order.len() / 2];
        }
    }

    println!("Correct Middle Total: {}", correct_middle_total);
    println!("Incorrect Middle Total: {}", incorrect_middle_total);
    Ok(())
}

fn is_good_order(update_order: &[u32], rules: &[[u32; 2]]) -> bool {
    for rule in rules {
        let [before_val, after_val] = rule;
        if update_order.contains(before_val) && update_order.contains(after_val) {
            let before_index = update_order
                .iter()
                .position(|val| val == before_val)
                .unwrap();
            let after_index = update_order
                .iter()
                .position(|val| val == after_val)
                .unwrap();
            if before_index > after_index {
                return false;
            }
        }
    }
    true
}

fn fix_update_order(update_order: &[u32], rules: &[[u32; 2]]) -> Vec<u32> {
    let mut new_order = Vec::new();
    for val in update_order {
        if new_order.is_empty() {
            new_order.push(*val);
        } else {
            let mut min_index_bound = 0;
            for rule in rules {
                let [before_val, after_val] = rule;
                if val == after_val && new_order.contains(before_val) {
                    let new_min_index = new_order.iter().position(|v| v == before_val).unwrap() + 1;
                    if new_min_index > min_index_bound {
                        min_index_bound = new_min_index;
                    }
                }
            }
            new_order.insert(min_index_bound, *val);
        }
    }

    new_order
}
