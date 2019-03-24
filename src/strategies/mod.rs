#[cfg(test)]
mod test;

pub mod hunt_food;

#[derive(Debug, PartialEq)]
pub struct Location {
    row: i16,
    col: i16,
}

impl Location {
    pub fn new(row: i16, col: i16) -> Self {
        Location { row, col }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
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
pub trait Strategy {
    fn run(&mut self, ants: Vec<Ant>, food: Vec<Food>) -> Vec<Command>;
}
