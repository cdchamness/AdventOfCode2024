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
    let farmland = Grid::new(input_grid);

    // Find Regions
    let t0 = std::time::Instant::now();
    let (index1_max, index2_max) = farmland.get_bounds();
    let mut regions: Vec<Region> = Vec::new();
    for i in 0..index1_max {
        for j in 0..index2_max {
            let next_gp = GridPoint::new(i as i32, j as i32);
            if !regions.iter().any(|reg| reg.contains(&next_gp)) {
                if let Some(crop_type) = farmland.get(&next_gp) {
                    let mut new_region = Region::new(next_gp, crop_type);
                    new_region.get_extent(&farmland);
                    regions.push(new_region);
                }
            }
        }
    }
    println!(
        "get_extent(): {}ms",
        (std::time::Instant::now() - t0).as_millis()
    );

    let t1 = std::time::Instant::now();
    let (index1_max, index2_max) = farmland.get_bounds();
    let mut regions: Vec<Region> = Vec::new();
    for i in 0..index1_max {
        for j in 0..index2_max {
            let next_gp = GridPoint::new(i as i32, j as i32);
            if !regions.iter().any(|reg| reg.contains(&next_gp)) {
                if let Some(crop_type) = farmland.get(&next_gp) {
                    let mut new_region = Region::new(next_gp, crop_type);
                    new_region.get_extent2(&farmland);
                    regions.push(new_region);
                }
            }
        }
    }
    println!(
        "get_extent2(): {}ms",
        (std::time::Instant::now() - t1).as_millis()
    );

    // Calcuate the total cost: (Area * perimeter)
    let mut total_cost = 0;
    for region in regions.clone() {
        total_cost += region.get_area() * region.get_perimeter();
    }
    println!("Total Cost: {}", total_cost);

    // Calculate the bulk discounted cost: (Area * sides)
    let mut discounted_cost = 0;
    for region in regions {
        discounted_cost += region.get_area() * region.count_sides();
    }
    println!("Discounted Cost: {}", discounted_cost);

    Ok(())
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

#[derive(Debug, Clone)]
struct Region {
    points: Vec<GridPoint>,
    crop_type: char,
}

impl Region {
    fn new(new_point: GridPoint, crop_type: char) -> Region {
        Region {
            points: vec![new_point],
            crop_type,
        }
    }

    fn contains(&self, x: &GridPoint) -> bool {
        self.points.contains(x)
    }

    fn get_extent(&mut self, farmland: &Grid) {
        // This finds all connected points and adds them to self.points
        let mut added_new_points = true;
        let mut new_points = Vec::new();

        while added_new_points {
            added_new_points = false;
            for point in &self.points {
                for direction in Direction::ALL {
                    let next_point = point + &direction.step();
                    if !self.points.contains(&next_point)
                        && !new_points.contains(&next_point)
                        && Some(self.crop_type) == farmland.get(&next_point)
                    {
                        added_new_points = true;
                        new_points.push(next_point);
                    }
                }
            }
            self.points.append(&mut new_points); // leaves new_points empty
        }
    }

    fn get_extent2(&mut self, farmland: &Grid) {
        // This finds all connected points and adds them to self.points
        let mut current_index = 0;
        while let Some(point) = self.points.clone().get(current_index) {
            for direction in Direction::ALL {
                let next_point = point + &direction.step();
                if !self.points.contains(&next_point)
                    && Some(self.crop_type) == farmland.get(&next_point)
                {
                    self.points.push(next_point);
                }
            }
            current_index += 1;
        }
    }

    fn get_area(&self) -> usize {
        self.points.len()
    }

    fn get_perimeter(&self) -> usize {
        let mut perimeter = 0;
        for point in &self.points {
            for direction in Direction::ALL {
                let neighbor_point = point + &direction.step();
                if !self.points.contains(&neighbor_point) {
                    perimeter += 1
                }
            }
        }
        perimeter
    }

    fn count_sides(&self) -> usize {
        let mut sides = 0;
        for direction in Direction::ALL {
            // points that share a side need to all be missing a neighbor in the same direction
            let mut side_points = self
                .points
                .clone()
                .into_iter()
                .filter(|p| !self.points.contains(&(p + &direction.step())))
                .collect::<Vec<GridPoint>>();
            match direction {
                Direction::Up | Direction::Down => {
                    // all points that share a side should have a matching index, here its index1
                    while let Some(current_point) = side_points.first() {
                        let same_height_points: Vec<GridPoint> = side_points
                            .clone()
                            .into_iter()
                            .filter(|p| p.index1 == current_point.index1)
                            .collect::<Vec<GridPoint>>();
                        // these should be safe to .expect() because same_height_points must have at least 1
                        // element, namely current_point. If side_points is empty then we are not in this while-loop
                        let left_point = same_height_points
                            .iter()
                            .min_by(|x, y| x.index2.cmp(&y.index2))
                            .expect("Could not find left-most point");
                        let right_point = same_height_points
                            .iter()
                            .max_by(|x, y| x.index2.cmp(&y.index2))
                            .expect("Could not find right-most point");
                        let gaps = Region::get_side_gaps(
                            &same_height_points,
                            left_point,
                            right_point,
                            Direction::Right,
                        );
                        sides += gaps + 1;
                        // remove the points we have just checked
                        side_points.retain_mut(|x| !same_height_points.contains(x));
                    }
                }
                Direction::Right | Direction::Left => {
                    // and here its index2
                    while let Some(current_point) = side_points.first() {
                        let same_width_points: Vec<GridPoint> = side_points
                            .clone()
                            .into_iter()
                            .filter(|p| p.index2 == current_point.index2)
                            .collect::<Vec<GridPoint>>();
                        // same as above, just a different direction
                        let top_point = same_width_points
                            .iter()
                            .min_by(|x, y| x.index1.cmp(&y.index1))
                            .expect("Could not find top-most point");
                        let bot_point = same_width_points
                            .iter()
                            .max_by(|x, y| x.index1.cmp(&y.index1))
                            .expect("Could not find bottom-most point");
                        let gaps = Region::get_side_gaps(
                            &same_width_points,
                            top_point,
                            bot_point,
                            Direction::Down,
                        );
                        sides += gaps + 1;
                        side_points.retain_mut(|x| !same_width_points.contains(x));
                    }
                }
            }
        }
        sides
    }

    fn get_side_gaps(
        point_list: &[GridPoint],
        start: &GridPoint,
        stop: &GridPoint,
        iter_dir: Direction,
    ) -> usize {
        if start == stop {
            // only 1 value => no gaps
            return 0;
        }
        // at least 1 step
        let mut gaps = 0;
        let mut prev_was_gap = false;
        let mut next_point = start + &iter_dir.step();
        while &next_point != stop {
            if !point_list.contains(&next_point) {
                if !prev_was_gap {
                    gaps += 1;
                }
                prev_was_gap = true;
            } else {
                prev_was_gap = false;
            }
            next_point = &next_point + &iter_dir.step();
        }
        gaps
    }
}
