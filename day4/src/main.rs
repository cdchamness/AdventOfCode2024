use anyhow::Result;

use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid { grid }
    }

    fn get(&self, location: &GridPoint) -> char {
        let bounds = GridPoint::new(self.grid.len() as i32, self.grid[0].len() as i32);
        if location.index1 >= bounds.index1
            || location.index2 >= bounds.index2
            || location.index1 < 0
            || location.index2 < 0
        {
            '.'
        } else {
            self.grid[location.index1 as usize][location.index2 as usize]
        }
    }

    fn find(&self, val: char) -> Vec<GridPoint> {
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

#[derive(Clone)]
struct GridPoint {
    index1: i32,
    index2: i32,
}

impl GridPoint {
    fn new(index1: i32, index2: i32) -> GridPoint {
        GridPoint { index1, index2 }
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
    let char_grid = Grid::new(input_grid);

    // FIND ALL XMASes
    let target = ['X', 'M', 'A', 'S'];
    let offsets = [
        GridPoint::new(1, 0),
        GridPoint::new(1, 1),
        GridPoint::new(0, 1),
        GridPoint::new(-1, 1),
        GridPoint::new(-1, 0),
        GridPoint::new(-1, -1),
        GridPoint::new(0, -1),
        GridPoint::new(1, -1),
    ];
    let mut found_count = 0;
    let x_locations = char_grid.find(target[0]);
    for x_location in x_locations {
        for offset in &offsets {
            let mut target_index = 1;
            let mut next_location = &x_location + offset;
            while target_index < target.len() {
                if char_grid.get(&next_location) == target[target_index] {
                    next_location = &next_location + offset;
                    target_index += 1;
                    if target_index == target.len() {
                        found_count += 1;
                    }
                } else {
                    break;
                }
            }
        }
    }

    // FIND ALL X-MASes
    let x_pattern_offsets = [
        GridPoint::new(1, 1),
        GridPoint::new(-1, -1),
        GridPoint::new(1, -1),
        GridPoint::new(-1, 1),
    ];

    let x_mas_solutions = [
        ['M', 'S', 'M', 'S'],
        ['M', 'S', 'S', 'M'],
        ['S', 'M', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
    ];

    let mut found_count2 = 0;
    let a_locations = char_grid.find('A');
    for a_location in a_locations {
        let mut x_pattern = [' '; 4];
        for (i, offset) in x_pattern_offsets.iter().enumerate() {
            x_pattern[i] = char_grid.get(&(&a_location + offset))
        }
        if x_mas_solutions.contains(&x_pattern) {
            found_count2 += 1;
        }
    }

    // Print Solutions
    println!("XMASes Found: {}", found_count);
    println!("X-MASes Found: {}", found_count2);

    Ok(())
}
