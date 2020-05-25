use super::super::dqn::Dqn;
use super::super::{Ant, Direction, Food, Location, Strategy};
use super::DqnStrategy;

use std::error::Error;
use std::result::Result;
use tensorflow::Tensor;
use tensorflow::TensorType;

struct TestDqn {
    width: u64,
    height: u64,
}

impl TestDqn {
    fn new(width: u64, height: u64) -> Self {
        TestDqn { width, height }
    }
}

impl Dqn for TestDqn {
    fn run<T: TensorType>(&mut self, _input: Tensor<T>) -> Result<Tensor<T>, Box<dyn Error>> {
        Err(From::from(""))
    }

    fn height(&self) -> u64 {
        self.height
    }

    fn width(&self) -> u64 {
        self.width
    }
}

#[test]
fn convert_input_should_return_tensor_with_correct_size() {
    let width = 50;
    let height = 40;
    let ant = Ant::new(Location::new(0, 0), 0);
    let food = vec![];
    let strategy = DqnStrategy::new(TestDqn::new(width, height));
    let tensor = strategy.convert_input(&ant, &food);

    let dims = tensor.dims();
    assert_eq!(dims[0], 1);
    assert_eq!(dims[1], 2);
    assert_eq!(dims[2], width);
    assert_eq!(dims[3], height);
}

#[test]
fn convert_input_should_return_tensor_with_first_layer_one_for_ant_position_zero_otherwise() {
    let col = 13;
    let row = 23;
    let ant = Ant::new(Location::new(row, col), 0);
    let food = vec![];
    let width = 50;
    let height = 40;
    let strategy = DqnStrategy::new(TestDqn::new(width, height));
    let tensor = strategy.convert_input(&ant, &food);

    for x in 0..width as i16 {
        for y in 0..height as i16 {
            let expected_result = if x == col && y == row { 1.0 } else { 0.0 };
            assert_eq!(
                tensor[width as usize * y as usize + x as usize],
                expected_result
            );
        }
    }
}

#[test]
fn convert_input_should_return_tensor_with_second_layer_one_for_each_food_position_zero_otherwise()
{
    let ant = Ant::new(Location::new(0, 0), 0);
    let food_col_1 = 13;
    let food_row_1 = 23;
    let food_col_2 = 7;
    let food_row_2 = 37;
    let food = vec![
        Food::new(Location::new(food_row_1, food_col_1)),
        Food::new(Location::new(food_row_2, food_col_2)),
    ];
    let width: usize = 50;
    let height: usize = 40;
    let strategy = DqnStrategy::new(TestDqn::new(width as u64, height as u64));
    let tensor = strategy.convert_input(&ant, &food);

    for x in 0..width as i16 {
        for y in 0..height as i16 {
            let expected_result =
                if (x == food_col_1 && y == food_row_1) || (x == food_col_2 && y == food_row_2) {
                    1.0
                } else {
                    0.0
                };
            assert_eq!(
                tensor[width * height + width * y as usize + x as usize],
                expected_result
            );
        }
    }
}

#[test]
fn convert_output_should_return_north_if_first_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[0] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40));
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::North);
}

#[test]
fn convert_output_should_return_east_if_second_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[1] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40));
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::East);
}

#[test]
fn convert_output_should_return_south_if_third_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[2] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40));
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::South);
}

#[test]
fn convert_output_should_return_west_if_fourth_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[3] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40));
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::West);
}

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = DqnStrategy::new(TestDqn::new(40, 50));
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}

#[test]
fn should_have_no_commands_on_list_with_only_enemy_ants() {
    let ants = vec![
        Ant::new(Location::new(3, 5), 1),
        Ant::new(Location::new(7, 11), 2),
    ];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = DqnStrategy::new(TestDqn::new(40, 50));
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}
