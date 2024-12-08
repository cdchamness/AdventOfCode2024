use anyhow::Result;

use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

fn main() -> Result<()> {
    // Read Input
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(input_file).lines();

    // Load to Grid
    let mut input_grid: Vec<Vec<char>> = Vec::new();
    for line in input_lines {
        match line {
            Ok(line_string) => {
                input_grid.push(line_string.chars().collect());
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    let map = Grid::new(input_grid);
    let bounds = map.get_bounds();
    let frequency_types = find_unique_frequencies(&map);

    let mut antenna_pairs = Vec::new();
    for frequency in frequency_types {
        let frequency_locations = map.find_all(frequency);
        for f1 in frequency_locations.clone() {
            for f2 in frequency_locations.clone() {
                if f1 != f2 {
                    antenna_pairs.push((f1.clone(), f2.clone()));
                }
            }
        }
    }

    // Find antinodes
    let mut antinode_locations = Vec::new();
    for pair in antenna_pairs.clone() {
        let (antinode1, antinode2) = get_antinodes(pair);
        if !antinode_locations.contains(&antinode1) && antinode1.is_in_bounds(bounds) {
            antinode_locations.push(antinode1);
        }
        if !antinode_locations.contains(&antinode2) && antinode2.is_in_bounds(bounds) {
            antinode_locations.push(antinode2);
        }
    }
    println!("Antinode locations: {}", antinode_locations.len());

    // Find antinodes with harmonics
    let mut antinode_locations_with_harmonics = Vec::new();
    for pair in antenna_pairs {
        let antinodes_from_pair = get_antinodes_with_harmonics(pair, bounds);
        for antinode in antinodes_from_pair {
            if !antinode_locations_with_harmonics.contains(&antinode) {
                antinode_locations_with_harmonics.push(antinode);
            }
        }
    }
    println!(
        "Antinode locations with harmonics: {}",
        antinode_locations_with_harmonics.len()
    );

    Ok(())
}

fn find_unique_frequencies(grid: &Grid) -> Vec<char> {
    let mut frequencies = Vec::new();
    let (index1_max, index2_max) = grid.get_bounds();
    for i in 0..index1_max {
        for j in 0..index2_max {
            if let Some(next_char) = grid.get(&GridPoint::new(i as i32, j as i32)) {
                if next_char != '.' && !frequencies.contains(&next_char) {
                    frequencies.push(next_char);
                }
            }
        }
    }
    frequencies
}

fn get_antinodes(pair: (GridPoint, GridPoint)) -> (GridPoint, GridPoint) {
    let (node1, node2) = pair;
    let gap = &node2 - &node1;
    (&node2 + &gap, &node1 - &gap)
}

fn get_antinodes_with_harmonics(
    pair: (GridPoint, GridPoint),
    bounds: (usize, usize),
) -> Vec<GridPoint> {
    let (node1, node2) = pair;
    let mut antinodes = Vec::new();
    let gap = &node2 - &node1;

    let mut add_gap_node = node2.clone();
    while add_gap_node.is_in_bounds(bounds) {
        antinodes.push(add_gap_node.clone());
        add_gap_node = &add_gap_node + &gap;
    }
    let mut sub_gap_node = node1.clone();
    while sub_gap_node.is_in_bounds(bounds) {
        antinodes.push(sub_gap_node.clone());
        sub_gap_node = &sub_gap_node - &gap;
    }

    antinodes
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid { grid }
    }

    fn get_bounds(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn get(&self, location: &GridPoint) -> Option<char> {
        let bounds = GridPoint::new(self.grid.len() as i32, self.grid[0].len() as i32);
        if location.index1 >= bounds.index1
            || location.index2 >= bounds.index2
            || location.index1 < 0
            || location.index2 < 0
        {
            // Out of Bounds
            None
        } else {
            Some(self.grid[location.index1 as usize][location.index2 as usize])
        }
    }

    fn find_all(&self, val: char) -> Vec<GridPoint> {
        let mut locations: Vec<GridPoint> = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            let js = row
                .iter()
                .enumerate()
                .map(|(j, &ch)| if ch == val { Some(j) } else { None })
                .collect::<Vec<Option<usize>>>();
            for j in js.into_iter().flatten() {
                locations.push(GridPoint::new(i as i32, j as i32))
            }
        }
        locations
    }
}

#[derive(Clone, Debug, PartialEq)]
struct GridPoint {
    index1: i32,
    index2: i32,
}

impl GridPoint {
    fn new(index1: i32, index2: i32) -> GridPoint {
        GridPoint { index1, index2 }
    }

    fn is_in_bounds(&self, bounds: (usize, usize)) -> bool {
        self.index1 < bounds.0 as i32
            && self.index2 < bounds.1 as i32
            && self.index1 >= 0
            && self.index2 >= 0
    }
}

impl Add<&GridPoint> for &GridPoint {
    type Output = GridPoint;
    fn add(self, rhs: &GridPoint) -> Self::Output {
        GridPoint {
            index1: self.index1 + rhs.index1,
            index2: self.index2 + rhs.index2,
        }
    }
}

impl Sub<&GridPoint> for &GridPoint {
    type Output = GridPoint;
    fn sub(self, rhs: &GridPoint) -> Self::Output {
        GridPoint {
            index1: self.index1 - rhs.index1,
            index2: self.index2 - rhs.index2,
        }
    }
}
