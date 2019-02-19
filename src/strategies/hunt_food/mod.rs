#[cfg(test)]
mod test;

use super::{Ant, Command, Food, Strategy};

pub struct HuntFoodStrategy {}

impl HuntFoodStrategy {
    pub fn new() -> Self {
        HuntFoodStrategy {}
    }
}

impl Strategy for HuntFoodStrategy {
    fn run(&mut self, _ants: Vec<Ant>, _food: Vec<Food>) -> Vec<Command> {
        Vec::new()
    }
}
