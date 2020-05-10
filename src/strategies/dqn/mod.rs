#[cfg(test)]
mod test;

use super::super::dqn::Dqn;
use super::{Ant, Command, Direction, Food, Strategy};

use tensorflow::Tensor;

pub struct DqnStrategy<T: Dqn> {
    dqn: T,
}

impl<T: Dqn> DqnStrategy<T> {
    pub fn new(dqn: T) -> Self {
        DqnStrategy { dqn }
    }

    pub fn convert_input(&self, ant: &Ant, food: &Vec<Food>) -> Tensor<f32> {
        let height = self.dqn.height();
        let width = self.dqn.width();
        let mut result = Tensor::new(&[1, 2, width, height]);
        let height = height as usize;
        let width = width as usize;
        let ant_index = width * ant.location.row as usize + ant.location.col as usize;
        result[ant_index] = 1.0;
        for f in food {
            let food_index =
                height * width + width * f.location.row as usize + f.location.col as usize;
            result[food_index] = 1.0;
        }
        result
    }
    pub fn convert_output(&self, tensor: Tensor<f32>) -> Direction {
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
}

impl<T: Dqn> Strategy for DqnStrategy<T> {
    fn run(&mut self, ants: &Vec<Ant>, food: &Vec<Food>) -> Vec<Command> {
        let mut result = Vec::new();
        for ant in ants {
            let input_tensor = self.convert_input(ant, food);
            let output_tensor = self.dqn.run(input_tensor).unwrap();
            let direction = self.convert_output(output_tensor);
            let command = Command::new(ant.location, direction);
            result.push(command);
        }
        result
    }
}
