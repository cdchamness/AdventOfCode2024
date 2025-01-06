use anyhow::Result;

use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
    process::exit,
};

fn main() -> Result<()> {
    // Read Input
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(&input_file).lines();

    // Load to Grid
    let mut input_grid: Vec<Vec<char>> = Vec::new();
    let mut instructions = String::new();
    for line in input_lines {
        match line {
            Ok(line_string) => {
                if line_string.starts_with('#') {
                    input_grid.push(line_string.chars().collect());
                } else {
                    instructions.push_str(&line_string);
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }

    // Simulate normal warehouse instructions
    let mut warehouse = Grid::new(input_grid);
    let mut robot_location = warehouse
        .find('@')
        .expect("Cannot find initial robot location");

    for instruction in instructions.chars() {
        match instruction {
            '>' => try_instruction(&mut warehouse, &mut robot_location, Direction::Right),
            '^' => try_instruction(&mut warehouse, &mut robot_location, Direction::Up),
            '<' => try_instruction(&mut warehouse, &mut robot_location, Direction::Left),
            'v' => try_instruction(&mut warehouse, &mut robot_location, Direction::Down),
            _ => {}
        }
    }

    let boxes_locations = warehouse.find_all('O');
    let gps_sum = boxes_locations
        .iter()
        .map(|box_loc| box_loc.get_gps_coordinates())
        .sum::<usize>();

    println!("{:?}", gps_sum);

    // Load scaled warehouse to grid
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(input_file).lines();

    let mut scaled_grid: Vec<Vec<char>> = Vec::new();
    for line in input_lines {
        match line {
            Ok(line_string) => {
                if line_string.starts_with('#') {
                    let mut row = Vec::new();
                    for ch in line_string.chars() {
                        match ch {
                            '#' => {
                                row.push('#');
                                row.push('#');
                            }
                            'O' => {
                                row.push('[');
                                row.push(']');
                            }
                            '.' => {
                                row.push('.');
                                row.push('.');
                            }
                            '@' => {
                                row.push('@');
                                row.push('.');
                            }
                            _ => exit(1),
                        }
                    }
                    scaled_grid.push(row);
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    let mut scaled_warehouse = Grid::new(scaled_grid);
    let mut scaled_robot_location = scaled_warehouse
        .find('@')
        .expect("Cannot find initial robot location");
    for instruction in instructions.chars() {
        // println!("Instruction: {}", instruction);
        // scaled_warehouse.display();
        // println!("\n\n");
        match instruction {
            // left and right are still the same
            '>' => try_instruction(
                &mut scaled_warehouse,
                &mut scaled_robot_location,
                Direction::Right,
            ),
            '<' => try_instruction(
                &mut scaled_warehouse,
                &mut scaled_robot_location,
                Direction::Left,
            ),
            // note that only the up and down logic is changed by the boxes being 2-wide
            '^' => try_scaled_instruction(
                &mut scaled_warehouse,
                &mut scaled_robot_location,
                Direction::Up,
            ),
            'v' => try_scaled_instruction(
                &mut scaled_warehouse,
                &mut scaled_robot_location,
                Direction::Down,
            ),
            _ => {}
        }
    }

    let scaled_boxes_locations = scaled_warehouse.find_all('[');
    let gps_sum2 = scaled_boxes_locations
        .iter()
        .map(|box_loc| box_loc.get_gps_coordinates())
        .sum::<usize>();

    println!("{:?}", gps_sum2);

    Ok(())
}

fn try_instruction(warehouse: &mut Grid, current_location: &mut GridPoint, direction: Direction) {
    let mut next_location = *current_location + &direction.step();
    let mut stack = vec!['.', '@'];
    while let Some(ch) = warehouse.get(&next_location) {
        if ch == '#' {
            // Found a wall before an open space => cannot push
            // => just exit without doing anything
            return;
        } else if ch == '.' {
            // Found an open space before a wall
            while let Some(pop_val) = stack.pop() {
                warehouse.set(&next_location, pop_val);
                next_location = next_location + &direction.opposite().step();
            }
            // update robots location and exit early again
            current_location.index1 += direction.step().index1;
            current_location.index2 += direction.step().index2;
            return;
        } else {
            // this means we found a box => add it to the list of what we are pushing
            stack.push(ch);
        }

        next_location = next_location + &direction.step();
    }
}

fn try_scaled_instruction(
    warehouse: &mut Grid,
    current_location: &mut GridPoint,
    direction: Direction,
) {
    let mut spaces_to_check = vec![*current_location + &direction.step()];
    let mut items_pushing = vec![('@', *current_location)];
    while let Some(loc) = spaces_to_check.pop() {
        if let Some(ch) = warehouse.get(&loc) {
            match ch {
                '#' => {
                    // found a wall => cannot push => exit
                    return;
                }
                '[' => {
                    // this means the other half of the box is on the right
                    let next_location = loc + &direction.step();

                    let this_side = ('[', loc);
                    if !items_pushing.contains(&this_side) {
                        items_pushing.push(this_side);
                        spaces_to_check.push(next_location);
                    }

                    let other_side = (']', loc + &Direction::Right.step());
                    if !items_pushing.contains(&other_side) {
                        items_pushing.push(other_side);
                        spaces_to_check.push(next_location + &Direction::Right.step());
                    }
                }
                ']' => {
                    // this means the other half of the box is on the left
                    let next_location = loc + &direction.step();

                    let this_side = (']', loc);
                    if !items_pushing.contains(&this_side) {
                        items_pushing.push(this_side);
                        spaces_to_check.push(next_location);
                    }

                    let other_side = ('[', loc + &Direction::Left.step());
                    if !items_pushing.contains(&other_side) {
                        items_pushing.push(other_side);
                        spaces_to_check.push(next_location + &Direction::Left.step());
                    }
                }
                '.' => {
                    // empty space, these boxes can move freely no other spaces to check from this loc
                }
                _ => {
                    eprintln!("Found an unknown symbol in try_scaled_instruction");
                    exit(2)
                }
            }
        }
    }
    // if we made it out of the while loop that means we are able to push
    // so we want to update the boxes locations from farthest to closest
    // sort puts in order from top to bottom
    //    => if going up that is what we want
    //    => if going down we want the reverse
    items_pushing.sort_unstable_by(|(_, p1), (_, p2)| p1.index1.cmp(&p2.index1));
    if direction == Direction::Down {
        items_pushing.reverse();
    }

    for &(ch, loc) in &items_pushing {
        // push the item
        warehouse.set(&(loc + &direction.step()), ch);
        // it leaves a space where it was
        warehouse.set(&loc, '.');
    }
    // update the robots location
    current_location.index1 += direction.step().index1;
    current_location.index2 += direction.step().index2;
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

    fn display(&self) {
        for row in &self.grid {
            let mut row_str = String::new();
            for ch in row {
                row_str.push(*ch);
            }
            println!("{}", row_str);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct GridPoint {
    index1: i32,
    index2: i32,
}

impl GridPoint {
    fn new(index1: i32, index2: i32) -> GridPoint {
        GridPoint { index1, index2 }
    }

    fn get_gps_coordinates(&self) -> usize {
        100 * self.index1 as usize + self.index2 as usize
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

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}
