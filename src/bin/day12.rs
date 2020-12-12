use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::error::Error;
use core::mem;

const DAY: &str = "day12";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {

    let mut instructions: Vec<(char, i32)> = Vec::with_capacity(input.len());

    for line in input {
        let mut line = line.chars();
        let char = line.next().ok_or("No input on line")?;
        let number: String = line.collect();
        let number: u32 = number.parse()?;
        let number: i32 = number as i32;
        instructions.push((char, number));
    }

    return Ok([run1(&instructions)?, run2(&instructions)?])
}

pub fn run1(input: &Vec<(char, i32)>) -> Result<Option<String>, Box<dyn Error>> {

    let mut position = Position {x: 0, y: 0, dir: Direction::East};

    for (char, number) in input {
        let number = *number;
        match char {
            'F' => Ok(position.forward(number)),
            'N' => Ok(position.dir_move(Direction::North, number)),
            'E' => Ok(position.dir_move(Direction::East, number)),
            'S' => Ok(position.dir_move(Direction::South, number)),
            'W' => Ok(position.dir_move(Direction::West, number)),
            'L' => position.left(number),
            'R' => position.right(number),
            c => Err(format!("Unknown input {}{}", c, number))?
        }?
    }
    let distance = position.x.abs() + position.y.abs();

    return Ok(Some(distance.to_string()));
}

pub fn run2(input: &Vec<(char, i32)>) -> Result<Option<String>, Box<dyn Error>> {

    let mut position = PositionWithWaypoint::new(0, 0, 10, 1);

    for (char, number) in input {
        let number = *number;
        match char {
            'F' => Ok(position.forward(number)),
            'N' => Ok(position.dir_move(Direction::North, number)),
            'E' => Ok(position.dir_move(Direction::East, number)),
            'S' => Ok(position.dir_move(Direction::South, number)),
            'W' => Ok(position.dir_move(Direction::West, number)),
            'L' => position.left(number),
            'R' => position.right(number),
            c => Err(format!("Unknown input {}", c))?
        }?
    }
    let distance = position.x.abs() + position.y.abs();

    return Ok(Some(distance.to_string()));
}


#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }


    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0,1),
            Direction::East => (1,0),
            Direction::South => (0,-1),
            Direction::West => (-1,0),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    fn dir_move(&mut self, dir: Direction, steps: i32) {
        let (dx, dy) = dir.delta();
        self.x += dx * steps;
        self.y += dy * steps;
    }

    fn forward(&mut self, steps: i32) {
        self.dir_move(self.dir, steps);
    }

    fn left(&mut self, degrees: i32) -> Result<(), String> {
        match degrees % 360 {
            90 => Ok(self.dir = self.dir.left()),
            180 => Ok(self.dir = self.dir.left().left()),
            270 => Ok(self.dir = self.dir.right()),
            0 => Ok(()),
            _ => Err(format!("Unknown amount of degrees {}", degrees)),
        }
    }

    fn right(&mut self, degrees: i32) -> Result<(), String> {
        self.left(360-degrees)
    }
}

struct PositionWithWaypoint {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl PositionWithWaypoint {
    fn new(self_x: i32, self_y: i32, dx: i32, dy: i32, ) -> Self {
        PositionWithWaypoint { x: self_x, y: self_y, dx, dy }
    }

    fn dir_move(&mut self, dir: Direction, steps: i32) {
        let (dx, dy) = dir.delta();
        self.dx += dx * steps;
        self.dy += dy * steps;
    }

    fn forward(&mut self, steps: i32) {
        self.x += self.dx * steps;
        self.y += self.dy * steps;
    }

    fn left(&mut self, degrees: i32) -> Result<(), String> {
        match degrees % 360 {
            90 => Ok({
                mem::swap(&mut self.dx, &mut self.dy);
                self.dx = -self.dx;
            }),
            180 => Ok({
                self.dx = -self.dx;
                self.dy = -self.dy;
            }),
            270 => Ok({
                mem::swap(&mut self.dx, &mut self.dy);
                self.dy = -self.dy;
            }),
            0 => Ok(()),
            _ => Err(format!("Unknown amount of degrees {}", degrees)),
        }
    }


    fn right(&mut self, degrees: i32) -> Result<(), String> {
        self.left(360-degrees)
    }
}

#[test]
pub fn test_day12() {
    assert!(common::run_test(DAY, &run))
}