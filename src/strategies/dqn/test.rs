use super::super::dqn::Dqn;
use super::super::{Food, Location, Strategy};
use super::DqnStrategy;

use std::error::Error;
use std::result::Result;
use tensorflow::Tensor;
use tensorflow::TensorType;

struct TestDqn {}

impl TestDqn {
    fn new() -> Self {
        TestDqn {}
    }
}

impl Dqn for TestDqn {
    fn run<T: TensorType>(&mut self, _input: Tensor<T>) -> Result<Tensor<T>, Box<dyn Error>> {
        Err(From::from(""))
    }
}

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = DqnStrategy::new(TestDqn::new());
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}
