#[cfg(test)]
mod test;

use super::super::dqn::Dqn;
use super::{Ant, Command, Direction, Food, Strategy};

use tensorflow::Tensor;

pub fn convert_input(ant: &Ant, food: &Vec<Food>) -> Tensor<f32> {
    let mut result = Tensor::new(&[2, 100, 100]);
    let ant_index = 100 * ant.location.row as usize + ant.location.col as usize;
    result[ant_index] = 1.0;
    for f in food {
        let food_index = 100 * 100 + 100 * f.location.row as usize + f.location.col as usize;
        result[food_index] = 1.0;
    }
    result
}

pub fn convert_output(tensor: Tensor<f32>) -> Direction {
    let max_index = tensor
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Found NaN in tensor output"))
        .map(|(index, _)| index);
    match max_index {
        Some(0) => Direction::North,
        Some(1) => Direction::East,
        Some(2) => Direction::South,
        Some(3) => Direction::West,
        Some(value) => panic!("Unknown tensor index: {}", value),
        None => panic!("Failed to find max in tensor output"),
    }
}

pub struct DqnStrategy<T: Dqn> {
    dqn: T,
}

impl<T: Dqn> DqnStrategy<T> {
    pub fn new(dqn: T) -> Self {
        DqnStrategy { dqn }
    }
}

impl<T: Dqn> Strategy for DqnStrategy<T> {
    fn run(&mut self, ants: &Vec<Ant>, food: &Vec<Food>) -> Vec<Command> {
        let mut result = Vec::new();
        for ant in ants {
            let input_tensor = convert_input(ant, food);
            if let Ok(output_tensor) = self.dqn.run(input_tensor) {
                let direction = convert_output(output_tensor);
                let command = Command::new(ant.location, direction);
                result.push(command);
            }
        }
        result
    }
}
