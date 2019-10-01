#[cfg(test)]
mod test;

use super::super::dqn::Dqn;
use super::{Ant, Command, Food, Strategy};

pub struct DqnStrategy<T: Dqn> {
    _dqn: T,
}

impl<T: Dqn> DqnStrategy<T> {
    pub fn new(dqn: T) -> Self {
        DqnStrategy { _dqn: dqn }
    }
}

impl<T: Dqn> Strategy for DqnStrategy<T> {
    fn run(&mut self, _ants: &Vec<Ant>, _food: &Vec<Food>) -> Vec<Command> {
        let result = Vec::new();
        // for ant in ants {
        // let input_tensor = convert_input(ant, ants, food);
        // let output_tensor = self.dqn.run(input_tensor);
        // let command = convert_output(output_tensor);
        // result.push(command);
        // }
        result
    }
}
