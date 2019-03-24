use super::super::{Ant, Command, Direction, Food, Location, Strategy};
use super::HuntFoodStrategy;

#[test]
fn should_have_no_commands_on_empty_ant_list() {
    let ants = vec![];
    let food = vec![Food::new(Location::new(1, 2))];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}

#[test]
fn should_have_no_commands_on_empty_food_list() {
    let ants = vec![Ant::new(Location::new(1, 2), 3)];
    let food = vec![];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 0);
}

#[test]
fn should_direct_ants_to_closest_food() {
    let ant1_row = 10;
    let ant1_col = 10;
    let ant2_row = 20;
    let ant2_col = 20;
    let ant3_row = 30;
    let ant3_col = 30;
    let ant4_row = 40;
    let ant4_col = 40;
    let ants = vec![
        Ant::new(Location::new(ant1_row, ant1_col), 0),
        Ant::new(Location::new(ant2_row, ant2_col), 0),
        Ant::new(Location::new(ant3_row, ant3_col), 0),
        Ant::new(Location::new(ant4_row, ant4_col), 0),
    ];

    let food = vec![
        Food::new(Location::new(ant1_row - 4, ant1_col + 0)),
        Food::new(Location::new(ant2_row + 0, ant2_col + 4)),
        Food::new(Location::new(ant3_row + 4, ant3_col + 0)),
        Food::new(Location::new(ant4_row + 0, ant4_col - 4)),
    ];

    let mut strategy = HuntFoodStrategy::new();
    let commands = strategy.run(&ants, &food);

    assert_eq!(commands.len(), 4);
    assert_eq!(
        commands[0],
        Command::new(Location::new(ant1_row, ant1_col), Direction::North)
    );
    assert_eq!(
        commands[1],
        Command::new(Location::new(ant2_row, ant2_col), Direction::East)
    );
    assert_eq!(
        commands[2],
        Command::new(Location::new(ant3_row, ant3_col), Direction::South)
    );
    assert_eq!(
        commands[3],
        Command::new(Location::new(ant4_row, ant4_col), Direction::West)
    );
}
