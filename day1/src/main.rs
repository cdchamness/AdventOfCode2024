use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

fn main() -> Result<()> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // let input_file = fs::File::open("puzzle_input_example.txt")?;
    let input_file = fs::File::open("puzzle_input.txt")?;
    let input_line = io::BufReader::new(input_file).lines();
    for line in input_line {
        match line {
            Ok(line_str) => {
                let mut line_iter = line_str.split_whitespace();
                if let Some(str_val) = line_iter.next() {
                    if let Ok(a) = str_val.parse::<u64>() {
                        list1.push(a);
                    }
                }
                if let Some(str_val) = line_iter.next() {
                    if let Ok(b) = str_val.parse::<u64>() {
                        list2.push(b);
                    }
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }

    // First way - Faster in the example by slower with the full list
    // let mut total_distance = 0;
    // while !list1.is_empty() {
    //     let a_min = get_smallest_value(&mut list1);
    //     let b_min = get_smallest_value(&mut list2);
    //     total_distance = {
    //         if a_min > b_min {
    //             total_distance + a_min - b_min
    //         } else {
    //             total_distance + b_min - a_min
    //         }
    //     }
    // }
    // println!("List Distance: {}", total_distance);

    // Second way
    let total_distance = get_list_distance(list1.clone(), list2.clone());
    println!("List Distance: {}", total_distance);

    let similarity_score = get_list_similarity(list1, list2);
    println!("Similarity Score: {}", similarity_score);

    Ok(())
}

// fn get_smallest_value(v: &mut Vec<u64>) -> u64 {
//     let mut min_index = 0;
//     let mut min_val = u64::MAX;
//     for (i, val) in v.iter().enumerate() {
//         if min_val > *val {
//             min_val = *val;
//             min_index = i;
//         }
//     }
//     v.remove(min_index)
// }

fn get_list_distance(mut a: Vec<u64>, mut b: Vec<u64>) -> u64 {
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b).fold(0, |sum, (a_val, b_val)| {
        if a_val > &b_val {
            sum + a_val - b_val
        } else {
            sum + b_val - a_val
        }
    })
}

fn get_list_similarity(a: Vec<u64>, b: Vec<u64>) -> u64 {
    let mut similarity_score = 0;
    for a_val in a {
        let score_increase = b.iter().fold(0, |total, b_val| {
            if a_val == *b_val {
                total + a_val
            } else {
                total
            }
        });
        similarity_score += score_increase;
    }
    similarity_score
}
