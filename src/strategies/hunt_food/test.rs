use super::super::{Ant, Command, Direction, Food, Strategy};
use super::HuntFoodStrategy;

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(1, 2)];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(ants, food);

    assert_eq!(commands.len(), 0);
}

#[test]
fn should_have_no_commands_on_empty_food_list() {
    let ants = vec![Ant::new(1, 2, 3)];
    let food = vec![];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(ants, food);

    assert_eq!(commands.len(), 0);
}

#[test]
fn should_direct_ants_to_closest_food() {
    let ant1_row = 20;
    let ant1_col = 20;
    let ant2_row = 20;
    let ant2_col = 30;
    let ants = vec![
        Ant::new(ant1_row, ant1_col, 0),
        Ant::new(ant2_row, ant2_col, 0),
    ];

    let food1_row = 10;
    let food1_col = 25;
    let food = vec![Food::new(food1_row, food1_col)];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(ants, food);

    assert_eq!(commands.len(), 2);
    assert_eq!(
        commands[0],
        Command::new(ant1_row, ant1_col, Direction::North)
    );
    assert_eq!(
        commands[1],
        Command::new(ant2_row, ant2_col, Direction::North)
    );
}
