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

    // Find all location the guard visits
    // Set initial conditions
    let mut char_grid = Grid::new(input_grid.clone());
    let mut guard_position = char_grid.find('^').expect("Unable to find guard location!");
    let mut guard_direction = Direction::Up;

    // Set the initial spot as visited
    char_grid.set(&guard_position, 'X');
    let mut next_char = char_grid.get(&(&guard_position + &guard_direction.step()));
    while next_char.is_some() {
        if next_char == Some('#') {
            // check if obstruction is in front
            // if so turn
            guard_direction = guard_direction.turn();
        } else {
            // if no obstruction in front
            // update guard position & set location as visited
            guard_position = &guard_position + &guard_direction.step();
            char_grid.set(&guard_position, 'X');
        }
        // update for looping
        next_char = char_grid.get(&(&guard_position + &guard_direction.step()));
    }

    let total_squares_visited = char_grid.find_all('X').len();
    println!("Number of Visited Positions: {}", total_squares_visited);

    // // Find all locations where an obstruction would create a loop
    // // We only need to check locations where the guard will actually walk,
    // // if he doesn't go somewhere normally, then adding an obstruction there does nothing
    let mut possible_positions = 0;
    let mut checked_positions = Vec::new();
    let static_char_grid = Grid::new(input_grid);
    let obstruction_locations = static_char_grid.find_all('#');
    let mut guard_position = static_char_grid
        .find('^')
        .expect("Unable to find guard location!");
    checked_positions.push(guard_position.clone()); // We know we cannot put one where the guard starts
    let mut guard_direction = Direction::Up;

    let mut next_char = static_char_grid.get(&(&guard_position + &guard_direction.step()));
    while next_char.is_some() {
        if next_char == Some('#') {
            // movement logic
            guard_direction = guard_direction.turn();
        } else {
            // if no obstruction in front check if adding one would make a loop
            // but only if we haven't checked that location already
            let new_ob_location = &guard_position + &guard_direction.step();
            if !checked_positions.contains(&new_ob_location) {
                if test_new_obstruction(&obstruction_locations, &guard_position, guard_direction) {
                    possible_positions += 1;
                }
                checked_positions.push(new_ob_location);
            }
            // update guard position and add to walked positions
            guard_position = &guard_position + &guard_direction.step();
        }
        // update for looping
        next_char = static_char_grid.get(&(&guard_position + &guard_direction.step()));
    }
    println!(
        "Number of positions to add Obstruction: {}",
        possible_positions
    );

    Ok(())
}

fn test_new_obstruction(
    obstruction_location: &[GridPoint],
    guard_location: &GridPoint,
    current_direction: Direction,
) -> bool {
    // pretend there is an obstruction directly ahead add see if we create a loop
    let mut next_location = guard_location.clone();
    let mut next_dir = current_direction.turn();
    let added_obstruction_location = guard_location + &current_direction.step();
    let mut already_visited_obstructions =
        vec![(added_obstruction_location.clone(), current_direction)];

    let mut obstruction_list = obstruction_location.to_vec();
    obstruction_list.push(added_obstruction_location.clone());
    let mut next_ob =
        find_next_obstruction_in_direction(&obstruction_list, &next_location, next_dir);
    while next_ob.is_some() {
        if already_visited_obstructions.contains(&(next_ob.clone().unwrap(), next_dir)) {
            already_visited_obstructions.push((next_ob.clone().unwrap(), next_dir));
            return true;
        }
        already_visited_obstructions.push((next_ob.clone().unwrap(), next_dir));
        next_location = &next_ob.unwrap() + &next_dir.opposite().step();
        next_dir = next_dir.turn();
        next_ob = find_next_obstruction_in_direction(&obstruction_list, &next_location, next_dir);
    }
    false
}

fn find_next_obstruction_in_direction(
    obstruction_list: &[GridPoint],
    starting_point: &GridPoint,
    direction: Direction,
) -> Option<GridPoint> {
    match direction {
        Direction::Up => {
            let candidates = obstruction_list
                .iter()
                .filter(|&gp| {
                    gp.index2 == starting_point.index2 && gp.index1 < starting_point.index1
                })
                .collect::<Vec<&GridPoint>>();
            let mut closest_dist = i32::MAX;
            let mut next_obstruction = None;
            for gp in candidates {
                let dist = starting_point.index1 - gp.index1;
                if dist < closest_dist {
                    closest_dist = dist;
                    next_obstruction = Some(gp.clone());
                }
            }
            next_obstruction
        }
        Direction::Right => {
            let candidates = obstruction_list
                .iter()
                .filter(|&gp| {
                    gp.index1 == starting_point.index1 && gp.index2 > starting_point.index2
                })
                .collect::<Vec<&GridPoint>>();
            let mut closest_dist = i32::MAX;
            let mut next_obstruction = None;
            for gp in candidates {
                let dist = gp.index2 - starting_point.index2;
                if dist < closest_dist {
                    closest_dist = dist;
                    next_obstruction = Some(gp.clone());
                }
            }
            next_obstruction
        }
        Direction::Down => {
            let candidates = obstruction_list
                .iter()
                .filter(|&gp| {
                    gp.index2 == starting_point.index2 && gp.index1 > starting_point.index1
                })
                .collect::<Vec<&GridPoint>>();
            let mut closest_dist = i32::MAX;
            let mut next_obstruction = None;
            for gp in candidates {
                let dist = gp.index1 - starting_point.index1;
                if dist < closest_dist {
                    closest_dist = dist;
                    next_obstruction = Some(gp.clone());
                }
            }
            next_obstruction
        }
        Direction::Left => {
            let candidates = obstruction_list
                .iter()
                .filter(|&gp| {
                    gp.index1 == starting_point.index1 && gp.index2 < starting_point.index2
                })
                .collect::<Vec<&GridPoint>>();
            let mut closest_dist = i32::MAX;
            let mut next_obstruction = None;
            for gp in candidates {
                let dist = starting_point.index2 - gp.index2;
                if dist < closest_dist {
                    closest_dist = dist;
                    next_obstruction = Some(gp.clone());
                }
            }
            next_obstruction
        }
    }
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

    fn find(&self, val: char) -> Option<GridPoint> {
        for (i, row) in self.grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|&v| v == val) {
                return Some(GridPoint::new(i as i32, j as i32));
            }
        }
        None
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
    fn step(&self) -> GridPoint {
        match self {
            Direction::Up => GridPoint::new(-1, 0),
            Direction::Right => GridPoint::new(0, 1),
            Direction::Down => GridPoint::new(1, 0),
            Direction::Left => GridPoint::new(0, -1),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}
