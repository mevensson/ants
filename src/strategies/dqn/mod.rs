#[cfg(test)]
mod test;

use super::super::dqn::Dqn;
use super::{Ant, Command, Direction, Food, Strategy};

use rand::Rng;
use tensorflow::Tensor;

pub struct DqnStrategy<T: Dqn> {
    dqn: T,
    exploration_factor: f64,
}

impl<T: Dqn> DqnStrategy<T> {
    pub fn new(dqn: T, exploration_factor: f64) -> Self {
        DqnStrategy {
            dqn,
            exploration_factor,
        }
    }

    pub fn convert_input(&self, ant: &Ant, food: &Vec<Food>) -> Tensor<f32> {
        let height = self.dqn.height();
        let width = self.dqn.width();
        let mut result = Tensor::new(&[1, 1, width, height]);
        let half_height = (height / 2) as i16;
        let half_width = (width / 2) as i16;
        for f in food {
            let food_x_offset = f.location.col - ant.location.col + half_width;
            let food_y_offset = f.location.row - ant.location.row + half_height;
            if food_x_offset >= 0
                && food_x_offset < width as i16
                && food_y_offset >= 0
                && food_y_offset < height as i16
            {
                let food_index = width as usize * food_y_offset as usize + food_x_offset as usize;
                result[food_index] = 1.0;
            }
        }
        result
    }
    pub fn convert_output(&self, tensor: Tensor<f32>) -> Direction {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let index = if x < self.exploration_factor {
            Some(rng.gen_range(0, 4))
        } else {
            tensor
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Found NaN in tensor output"))
                .map(|(index, _)| index)
        };
        match index {
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
            if ant.owner == 0 {
                let input_tensor = self.convert_input(ant, food);
                let output_tensor = self.dqn.run(input_tensor).unwrap();
                let direction = self.convert_output(output_tensor);
                let command = Command::new(ant.location, direction);
                result.push(command);
            }
        }
        result
    }
}
