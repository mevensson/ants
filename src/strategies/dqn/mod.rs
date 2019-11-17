#[cfg(test)]
mod test;

use super::super::dqn::Dqn;
use super::{Ant, Command, Food, Strategy};

use tensorflow::Tensor;

pub fn convert_input(ant: &Ant, food: &Vec<Food>) -> Tensor<i32> {
    let mut result = Tensor::new(&[2, 100, 100]);
    let ant_index = 100 * ant.location.row as usize + ant.location.col as usize;
    result[ant_index] = 1;
    for f in food {
        let food_index = 100 * 100 + 100 * f.location.row as usize + f.location.col as usize;
        result[food_index] = 1;
    }
    result
}

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
