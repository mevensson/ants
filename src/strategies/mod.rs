#[cfg(test)]
mod test;

pub mod dqn;
pub mod hunt_food;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    row: i16,
    col: i16,
}

impl Location {
    pub fn new(row: i16, col: i16) -> Self {
        Location { row, col }
    }

    pub fn distance_sq(first: &Self, second: &Self) -> i32 {
        let row_distance = (first.row - second.row) as i32;
        let col_distance = (first.col - second.col) as i32;
        row_distance * row_distance + col_distance * col_distance
    }

    pub fn closest_direction(first: &Self, second: &Self) -> Direction {
        let row_distance = second.row - first.row;
        let col_distance = second.col - first.col;
        if row_distance.abs() >= col_distance.abs() {
            if row_distance <= 0 {
                return Direction::North;
            } else {
                return Direction::South;
            }
        } else {
            if col_distance >= 0 {
                return Direction::East;
            } else {
                return Direction::West;
            }
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.row, self.col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ant {
    location: Location,
    owner: i16,
}

impl Ant {
    pub fn new(location: Location, owner: i16) -> Self {
        Ant { location, owner }
    }

    pub fn parse(string: &str) -> Self {
        let mut tokens = string.split_whitespace();
        let row = tokens.next().unwrap();
        let col = tokens.next().unwrap();
        let owner = tokens.next().unwrap();
        Ant::new(
            Location::new(row.parse().unwrap(), col.parse().unwrap()),
            owner.parse().unwrap(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Food {
    location: Location,
}

impl Food {
    pub fn new(location: Location) -> Self {
        Food { location }
    }

    pub fn parse(string: &str) -> Self {
        let mut tokens = string.split_whitespace();
        let row = tokens.next().unwrap();
        let col = tokens.next().unwrap();
        Food::new(Location::new(row.parse().unwrap(), col.parse().unwrap()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction_string = match self {
            Direction::North => "N",
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
        };
        write!(f, "{}", direction_string)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Command {
    location: Location,
    direction: Direction,
}

impl Command {
    pub fn new(location: Location, direction: Direction) -> Self {
        Command {
            location,
            direction,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "o {} {}", self.location, self.direction)
    }
}

pub trait Strategy {
    fn run(&mut self, ants: &Vec<Ant>, food: &Vec<Food>) -> Vec<Command>;
}
