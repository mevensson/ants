#[cfg(test)]
mod test;

use super::{Ant, Command, Food, Strategy};

pub struct DqnStrategy {
    dqn: Dqn;
}

impl DqnStrategy {
    pub fn new(dqn: Dqn) -> Self {
        DqnStrategy {
            dqn
        }
    }
}

impl Strategy for DqnStrategy {
    fn run(&mut self, ants: &Vec<Ant>, food: &Vec<Food>) -> Vec<Command> {
        let mut result = Vec::new();
        for ant in ants {
            let input_tensor = convert_input(ant, ants, food);
            let output_tensor = self.dqn.run(input_tensor);
            let command = convert_output(output_tensor);
            result.push(command);
        }
        result
    }
}
