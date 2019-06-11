use super::super::{Food, Location, Strategy};
use super::DqnStrategy;

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = DqnStrategy::new();
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}
