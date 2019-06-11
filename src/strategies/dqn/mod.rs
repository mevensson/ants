#[cfg(test)]
mod test;

use super::{Ant, Command, Food, Strategy};

pub struct DqnStrategy {}

impl DqnStrategy {
    pub fn new() -> Self {
        DqnStrategy {}
    }
}

impl Strategy for DqnStrategy {
    fn run(&mut self, _ants: &Vec<Ant>, _food: &Vec<Food>) -> Vec<Command> {
        Vec::new()
    }
}
