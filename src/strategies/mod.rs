#[cfg(test)]
mod test;

pub mod hunt_food;

#[derive(Debug, PartialEq)]
pub struct Ant {
    row: i16,
    col: i16,
    owner: i16,
}

impl Ant {
    pub fn new(row: i16, col: i16, owner: i16) -> Self {
        Ant { row, col, owner }
    }

    pub fn parse(string: &str) -> Self {
        let mut tokens = string.split_whitespace();
        let row = tokens.next().unwrap();
        let col = tokens.next().unwrap();
        let owner = tokens.next().unwrap();
        Ant {
            row: row.parse().unwrap(),
            col: col.parse().unwrap(),
            owner: owner.parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Food {
    row: i16,
    col: i16,
}

impl Food {
    pub fn new(row: i16, col: i16) -> Self {
        Food { row, col }
    }

    pub fn parse(string: &str) -> Self {
        let mut tokens = string.split_whitespace();
        let row = tokens.next().unwrap();
        let col = tokens.next().unwrap();
        Food {
            row: row.parse().unwrap(),
            col: col.parse().unwrap(),
        }
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
    row: i16,
    col: i16,
    direction: Direction,
}

impl Command {
    pub fn new(row: i16, col: i16, direction: Direction) -> Self {
        Command { row, col, direction }
    }
}
pub trait Strategy {
    fn run(&mut self, ants: Vec<Ant>, food: Vec<Food>) -> Vec<Command>;
}
