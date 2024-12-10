use anyhow::Result;

use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
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

    // Calculate score - Sum of ( unique summits / trailhead )
    let mut total_score = 0;
    let topographical_map = Grid::new(input_grid);
    let trailheads = topographical_map.find_all('0');
    for trailhead in trailheads.clone() {
        let summit_locations: Vec<GridPoint> = find_all_summits(trailhead, &topographical_map);
        total_score += summit_locations.len();
    }
    println!("Total Score: {}", total_score);

    // Calculate rating - Sum of ( unique paths / trailhead )
    let mut total_rating = 0;
    for trailhead in trailheads {
        // this should be the same as before, but we just include every path
        // so no skipping already visited locations or summits
        let trailhead_rating = find_rating(trailhead, &topographical_map);
        total_rating += trailhead_rating;
    }
    println!("Total Rating: {}", total_rating);

    Ok(())
}

fn find_all_summits(start: GridPoint, topographical_map: &Grid) -> Vec<GridPoint> {
    let mut summits = Vec::new();
    let mut locations = vec![start];
    let mut already_visited = Vec::new();
    let directions = Direction::ALL;
    while let Some(current_point) = locations.pop() {
        already_visited.push(current_point.clone());
        if let Some(current_elevation) = topographical_map.get(&current_point) {
            if current_elevation == '9' && !summits.contains(&current_point) {
                summits.push(current_point);
            } else {
                for direction in directions {
                    let next_location = &current_point + &direction.step();
                    if let Some(next_elevation) = topographical_map.get(&next_location) {
                        if !already_visited.contains(&next_location)
                            && current_elevation
                                .to_string()
                                .parse::<u8>()
                                .expect("Could not parse elevation to digit")
                                + 1
                                == next_elevation
                                    .to_string()
                                    .parse::<u8>()
                                    .expect("Could not parse elevation to digit")
                            && !locations.contains(&next_location)
                        {
                            locations.push(next_location)
                        }
                    }
                }
            }
        }
    }

    summits
}

fn find_rating(start: GridPoint, topographical_map: &Grid) -> usize {
    let mut summit_paths_found = 0;
    let mut locations = vec![start];
    let directions = Direction::ALL;
    while let Some(current_point) = locations.pop() {
        if let Some(current_elevation) = topographical_map.get(&current_point) {
            if current_elevation == '9' {
                summit_paths_found += 1
            } else {
                for direction in directions {
                    let next_location = &current_point + &direction.step();
                    if let Some(next_elevation) = topographical_map.get(&next_location) {
                        if current_elevation
                            .to_string()
                            .parse::<u8>()
                            .expect("Could not parse elevation to digit")
                            + 1
                            == next_elevation
                                .to_string()
                                .parse::<u8>()
                                .expect("Could not parse elevation to digit")
                        {
                            locations.push(next_location);
                        }
                    }
                }
            }
        }
    }
    summit_paths_found
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        Grid { grid }
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

#[derive(Clone, Debug, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn step(&self) -> GridPoint {
        match self {
            Direction::Up => GridPoint::new(-1, 0),
            Direction::Right => GridPoint::new(0, 1),
            Direction::Down => GridPoint::new(1, 0),
            Direction::Left => GridPoint::new(0, -1),
        }
    }
}
