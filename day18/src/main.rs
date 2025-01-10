use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    ops::Add,
};

fn main() -> Result<()> {
    // example values
    // const BYTE_COUNT: usize = 12;
    // const GRID_SIZE: usize = 7;
    // let input_file = fs::File::open("puzzle_input_example.txt")?;

    // problem values
    const BYTE_COUNT: usize = 1024;
    const GRID_SIZE: usize = 71;
    let input_file = fs::File::open("puzzle_input.txt")?;

    let input_lines = BufReader::new(input_file).lines();

    let mut corrupted_coords = Vec::new();
    for line in input_lines {
        match line {
            Ok(coordinates) => {
                let vals = coordinates
                    .splitn(2, ',')
                    .filter_map(|a| a.parse::<i32>().ok())
                    .collect::<Vec<i32>>();
                corrupted_coords.push(GridPoint::new(vals[0], vals[1]))
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    let mut memory_grid = Grid::new(vec![vec!['.'; GRID_SIZE]; GRID_SIZE]);
    for corrupted in corrupted_coords.iter().take(BYTE_COUNT) {
        memory_grid.set(corrupted, '#');
    }

    // Part 1
    let mut minimum_steps = find_path(
        &memory_grid,
        GridPoint::new(0, 0),
        GridPoint::new(GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1),
    );
    println!(
        "Minimum steps: {}",
        minimum_steps.expect("No original path found!")
    );

    // Part 2
    let mut current_index = BYTE_COUNT - 1; // last point we added
    while minimum_steps.is_some() {
        // Exit as soon as we could not find a path
        // This is slow, (I should be checking if the byte drops in the current path
        // And reusing work from the previous step) but it is fast enough
        current_index += 1; // increment first so our index is correct after the loop
        memory_grid.set(&corrupted_coords[current_index], '#');
        minimum_steps = find_path(
            &memory_grid,
            GridPoint::new(0, 0),
            GridPoint::new(GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1),
        );
    }
    // Our 'current_index' points to the last byte we added before a path could not be found
    let final_dropped_byte_location = corrupted_coords
        .get(current_index)
        .expect("Could not get final byte location");

    println!(
        "Blocking byte location: {},{}",
        final_dropped_byte_location.index1, final_dropped_byte_location.index2
    );

    Ok(())
}

fn find_path(memory_grid: &Grid, start: GridPoint, stop: GridPoint) -> Option<usize> {
    let mut steps_map: HashMap<GridPoint, usize> = HashMap::new();
    steps_map.insert(start, 0);
    let mut queue = vec![start];
    while let Some(loc) = queue.pop() {
        let next_step_count = steps_map
            .get(&loc)
            .expect("Current location not in steps_map")
            + 1;
        for direction in Direction::ALL {
            let next_location = loc + &direction.step();
            if memory_grid.get(&next_location) == Some('.') {
                if let Some(steps) = steps_map.get_mut(&next_location) {
                    if next_step_count < *steps {
                        *steps = next_step_count;
                        queue.push(next_location);
                    }
                } else {
                    steps_map.insert(next_location, next_step_count);
                    queue.push(next_location);
                }
            }
        }
    }

    steps_map.get(&stop).cloned()
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

    fn set(&mut self, location: &GridPoint, val: char) {
        let bounds = GridPoint::new(self.grid.len() as i32, self.grid[0].len() as i32);
        if location.index1 < bounds.index1
            || location.index2 > bounds.index2
            || location.index1 >= 0
            || location.index2 >= 0
        {
            self.grid[location.index1 as usize][location.index2 as usize] = val
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridPoint {
    index1: i32,
    index2: i32,
}

impl GridPoint {
    fn new(index1: i32, index2: i32) -> GridPoint {
        GridPoint { index1, index2 }
    }
}

impl Add<&GridPoint> for GridPoint {
    type Output = GridPoint;
    fn add(self, rhs: &GridPoint) -> Self::Output {
        GridPoint {
            index1: self.index1 + rhs.index1,
            index2: self.index2 + rhs.index2,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
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
