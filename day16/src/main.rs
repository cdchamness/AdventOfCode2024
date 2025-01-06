use anyhow::Result;

use std::{
    char,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

fn main() -> Result<()> {
    // Read Input
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(&input_file).lines();

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

    let maze = Grid::new(input_grid);
    let reindeer_start = Reindeer::new(
        Direction::Right,
        maze.find('S').expect("Cannot find start location"),
    );

    let mut active_search: Vec<Reindeer> = vec![reindeer_start];

    let mut maze_score: HashMap<Reindeer, usize> = HashMap::new();
    maze_score.insert(reindeer_start, 0);

    while let Some(reindeer) = active_search.pop() {
        for direction in reindeer.facing_direction.get_possible_step_directions() {
            if let Some((next_reindeer, score_increase)) = reindeer.take_step(direction, &maze) {
                let new_score = maze_score
                    .get(&reindeer)
                    .expect("No score for starting reindeer!")
                    + score_increase;
                if let Some(best_score) = maze_score.get_mut(&next_reindeer) {
                    if new_score < *best_score {
                        *best_score = new_score;
                        if !active_search.contains(&next_reindeer) {
                            active_search.push(next_reindeer);
                        }
                    }
                } else {
                    maze_score.insert(next_reindeer, new_score);
                    active_search.push(next_reindeer);
                }
            }
        }
    }

    let end_location = maze.find('E').expect("Cannot find End location");
    let min_final_score = maze_score
        .iter()
        .filter_map(|(k, v)| {
            if k.location == end_location {
                Some(v)
            } else {
                None
            }
        })
        .min()
        .expect("Could not find minimum value");

    println!("Min Score: {}", min_final_score);

    let mut reindeer_queue = maze_score
        .iter()
        .filter_map(|(reindeer, score)| {
            if reindeer.location == end_location && score == min_final_score {
                Some(*reindeer)
            } else {
                None
            }
        })
        .collect::<Vec<Reindeer>>();
    let mut possible_seat_locations = reindeer_queue.clone();

    while let Some(reindeer) = reindeer_queue.pop() {
        let prev = find_previous(reindeer, &maze_score);
        for prev_reindeer in prev {
            possible_seat_locations.push(prev_reindeer);
            reindeer_queue.push(prev_reindeer)
        }
    }

    let mut unique_grid_points = Vec::new();
    for reindeer in possible_seat_locations {
        if !unique_grid_points.contains(&reindeer.location) {
            unique_grid_points.push(reindeer.location)
        }
    }

    println!("Number of Seat Locations: {}", unique_grid_points.len());
    Ok(())
}

fn find_previous(current_reindeer: Reindeer, scores: &HashMap<Reindeer, usize>) -> Vec<Reindeer> {
    let mut prev_reindeers = Vec::new();
    let previous_location =
        current_reindeer.location + &current_reindeer.facing_direction.opposite().step();

    let current_score = scores
        .get(&current_reindeer)
        .expect("Cannot find current Reindeer's score");

    let potential_prev_reindeer = scores
        .keys()
        .filter(|r| r.location == previous_location)
        .collect::<Vec<&Reindeer>>();

    for potential in potential_prev_reindeer {
        if let Some(potential_score) = scores.get(potential) {
            if (potential.facing_direction == current_reindeer.facing_direction
                && current_score - potential_score == 1)
                || (potential.facing_direction != current_reindeer.facing_direction
                    && current_score - potential_score == 1001)
            {
                prev_reindeers.push(*potential);
            }
        }
    }

    prev_reindeers
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

    fn find(&self, val: char) -> Option<GridPoint> {
        for (i, row) in self.grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|&v| v == val) {
                return Some(GridPoint::new(i as i32, j as i32));
            }
        }
        None
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
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn step(&self) -> GridPoint {
        match self {
            Direction::Up => GridPoint::new(-1, 0),
            Direction::Right => GridPoint::new(0, 1),
            Direction::Down => GridPoint::new(1, 0),
            Direction::Left => GridPoint::new(0, -1),
        }
    }

    fn get_possible_step_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Up, Direction::Left, Direction::Right],
            Direction::Right => vec![Direction::Right, Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Down, Direction::Right, Direction::Left],
            Direction::Left => vec![Direction::Left, Direction::Down, Direction::Up],
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Reindeer {
    facing_direction: Direction,
    location: GridPoint,
}

impl Reindeer {
    fn new(facing_direction: Direction, location: GridPoint) -> Reindeer {
        Reindeer {
            facing_direction,
            location,
        }
    }

    fn take_step(&self, next_dir: Direction, maze: &Grid) -> Option<(Reindeer, usize)> {
        let next_location = self.location + &next_dir.step();
        let score_increase = if next_dir == self.facing_direction {
            1
        } else {
            1001
        };
        if maze.get(&next_location) == Some('#') {
            None
        } else {
            Some((Reindeer::new(next_dir, next_location), score_increase))
        }
    }
}
