use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

fn main() -> Result<()> {
    // let input_file = File::open("puzzle_input_example.txt")?;
    let input_file = File::open("puzzle_input.txt")?;
    let input_line = BufReader::new(input_file).lines();

    let mut robots = Vec::new();
    for line in input_line {
        match line {
            Ok(levels_string) => {
                if let Some((pos_string, vel_string)) = levels_string.split_once(" ") {
                    let position = parse_position(pos_string);
                    let velocity = parse_velocity(vel_string);
                    robots.push(Robot::new(position, velocity));
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    let mut init_config = robots.clone();

    // let room = Vector2D::new(11, 7); // given written in problem for example
    let room = Vector2D::new(101, 103); // given written in problem for actual input
    for _seconds in 0..100 {
        for robot in &mut robots {
            robot.step(&room)
        }
    }

    let q1_bots = robots
        .iter()
        .filter(|r| r.get_quadrant_loc(&room) == Quadrant::Q1)
        .count();
    let q2_bots = robots
        .iter()
        .filter(|r| r.get_quadrant_loc(&room) == Quadrant::Q2)
        .count();
    let q3_bots = robots
        .iter()
        .filter(|r| r.get_quadrant_loc(&room) == Quadrant::Q3)
        .count();
    let q4_bots = robots
        .iter()
        .filter(|r| r.get_quadrant_loc(&room) == Quadrant::Q4)
        .count();
    let safety_factor = q1_bots * q2_bots * q3_bots * q4_bots;

    println!("Safety Factor: {}", safety_factor);

    for seconds in 0..10000 {
        display_config(seconds, &init_config, &room);
        for robot in &mut init_config {
            robot.step(&room);
        }
    }

    Ok(())
}

fn parse_position(pos_str: &str) -> Vector2D {
    let cleaned = pos_str.trim_start_matches("p=");
    let (x_str, y_str) = cleaned
        .split_once(",")
        .expect("Failed to parse position data, comma");
    let x = x_str
        .parse::<i32>()
        .expect("Failed to parse position data, x");
    let y = y_str
        .parse::<i32>()
        .expect("Failed to parse position data, y");
    Vector2D { x, y }
}

fn parse_velocity(vel_str: &str) -> Vector2D {
    let cleaned = vel_str.trim_start_matches("v=");
    let (x_str, y_str) = cleaned
        .split_once(",")
        .expect("Failed to parse position data, comma");
    let x = x_str
        .parse::<i32>()
        .expect("Failed to parse position data, x");
    let y = y_str
        .parse::<i32>()
        .expect("Failed to parse position data, y");
    Vector2D { x, y }
}

fn display_config(second: usize, robots: &[Robot], room_size: &Vector2D) {
    println!("seconds: {}", second);
    for y in 0..room_size.y {
        let mut row = String::new();
        for x in 0..room_size.x {
            let this_pos = Vector2D::new(x, y);
            let robots_here = robots.iter().filter(|r| r.pos == this_pos).count();
            if robots_here == 0 {
                row.push('.');
            } else {
                row.push('X');
            }
        }
        println!("{}", row);
    }
}

#[derive(Debug, PartialEq)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    fn new(x: i32, y: i32) -> Vector2D {
        Vector2D { x, y }
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct Robot {
    pos: Vector2D,
    vel: Vector2D,
}

impl Robot {
    fn new(pos: Vector2D, vel: Vector2D) -> Robot {
        Robot { pos, vel }
    }

    fn step(&mut self, room_size: &Vector2D) {
        let next_pos = self.pos + self.vel;
        self.pos.x = next_pos.x.rem_euclid(room_size.x);
        self.pos.y = next_pos.y.rem_euclid(room_size.y);
    }

    fn get_quadrant_loc(&self, room_size: &Vector2D) -> Quadrant {
        let x_split = room_size.x / 2;
        let y_split = room_size.y / 2;
        if self.pos.x > x_split && self.pos.y < y_split {
            Quadrant::Q1
        } else if self.pos.x < x_split && self.pos.y < y_split {
            Quadrant::Q2
        } else if self.pos.x < x_split && self.pos.y > y_split {
            Quadrant::Q3
        } else if self.pos.x > x_split && self.pos.y > y_split {
            Quadrant::Q4
        } else {
            Quadrant::None
        }
    }
}
