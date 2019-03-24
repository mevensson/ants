#[cfg(test)]
mod test;

use super::{Ant, Command, Direction, Food, Strategy};

pub struct HuntFoodStrategy {}

impl HuntFoodStrategy {
    pub fn new() -> Self {
        HuntFoodStrategy {}
    }
}

impl Strategy for HuntFoodStrategy {
    fn run(&mut self, ants: Vec<Ant>, food: Vec<Food>) -> Vec<Command> {
        let mut result = Vec::new();
        if !food.is_empty() {
            for ant in ants {
                result.push(Command::new(ant.row, ant.col, Direction::North));
            }
        }
        result
    }
}
