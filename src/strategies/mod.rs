#[cfg(test)]
mod test;

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

pub trait Strategy {
    fn run(&mut self, ant: Vec<Ant>, food: Vec<Food>);
}
