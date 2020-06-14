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
    let strategy = DqnStrategy::new(TestDqn::new(width, height), 0.0);
    let tensor = strategy.convert_input(&ant, &food);

    let dims = tensor.dims();
    assert_eq!(dims[0], 1);
    assert_eq!(dims[1], 1);
    assert_eq!(dims[2], width);
    assert_eq!(dims[3], height);
}

#[test]
fn convert_input_should_return_empty_tensor_if_no_food() {
    let ant = Ant::new(Location::new(13, 23), 0);
    let food = vec![];
    let width = 50;
    let height = 40;
    let strategy = DqnStrategy::new(TestDqn::new(width, height), 0.0);
    let tensor = strategy.convert_input(&ant, &food);

    for x in 0..width as i16 {
        for y in 0..height as i16 {
            assert_eq!(tensor[width as usize * y as usize + x as usize], 0.0);
        }
    }
}

#[test]
fn convert_input_should_return_tensor_with_one_for_each_food_position_relative_ant_zero_otherwise()
{
    let ant_col = 15;
    let ant_row = 20;
    let ant = Ant::new(Location::new(ant_row, ant_col), 0);
    let food_left_col = 5;
    let food_right_col = 24;
    let food_top_row = 0;
    let food_bottom_row = 39;
    let food = vec![
        Food::new(Location::new(food_top_row, food_left_col)),
        Food::new(Location::new(food_bottom_row, food_right_col)),
        Food::new(Location::new(food_top_row - 1, food_left_col)), // above
        Food::new(Location::new(food_bottom_row + 1, food_right_col)), // below
        Food::new(Location::new(food_top_row, food_left_col - 1)), // too left
        Food::new(Location::new(food_bottom_row, food_right_col + 1)), // too right
    ];
    let width: usize = 20;
    let height: usize = 40;
    let strategy = DqnStrategy::new(TestDqn::new(width as u64, height as u64), 0.0);
    let tensor = strategy.convert_input(&ant, &food);

    let half_height = (height / 2) as i16;
    let half_width = (width / 2) as i16;
    let food_x_offset_1 = food_left_col - ant_col + half_width;
    let food_y_offset_1 = food_top_row - ant_row + half_height;
    let food_x_offset_2 = food_right_col - ant_col + half_width;
    let food_y_offset_2 = food_bottom_row - ant_row + half_height;
    for x in 0..width as i16 {
        for y in 0..height as i16 {
            let expected_result = if (x == food_x_offset_1 && y == food_y_offset_1)
                || (x == food_x_offset_2 && y == food_y_offset_2)
            {
                1.0
            } else {
                0.0
            };
            let index = width * y as usize + x as usize;
            assert_eq!(
                tensor[index as usize], expected_result,
                "index = {}, x = {}, y = {}",
                index, x, y
            );
        }
    }
}

#[test]
fn convert_output_should_return_north_if_first_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[0] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40), 0.0);
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::North);
}

#[test]
fn convert_output_should_return_east_if_second_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[1] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40), 0.0);
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::East);
}

#[test]
fn convert_output_should_return_south_if_third_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[2] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40), 0.0);
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::South);
}

#[test]
fn convert_output_should_return_west_if_fourth_position_is_largest() {
    let mut tensor = Tensor::new(&[4]);
    tensor[3] = 0.1;

    let strategy = DqnStrategy::new(TestDqn::new(50, 40), 0.0);
    let direction = strategy.convert_output(tensor);

    assert_eq!(direction, Direction::West);
}

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = DqnStrategy::new(TestDqn::new(40, 50), 0.0);
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

    let mut strategy = DqnStrategy::new(TestDqn::new(40, 50), 0.0);
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}
