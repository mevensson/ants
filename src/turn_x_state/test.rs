use crate::state_machine::State;
use crate::strategies::{Ant, Command, Direction, Food, Location, Strategy};

use super::TurnXState;

use std::io::prelude::*;
use std::io::BufReader;

struct TestStrategy<'a> {
    ants: Option<&'a mut Vec<Ant>>,
    food: Option<&'a mut Vec<Food>>,
    commands: Option<Vec<Command>>,
}

impl<'a> TestStrategy<'a> {
    fn new(
        ants: Option<&'a mut Vec<Ant>>,
        food: Option<&'a mut Vec<Food>>,
        commands: Option<Vec<Command>>,
    ) -> Self {
        TestStrategy {
            ants,
            food,
            commands,
        }
    }
}

impl<'a> Strategy for TestStrategy<'a> {
    fn run(&mut self, new_ants: &Vec<Ant>, new_food: &Vec<Food>) -> Vec<Command> {
        match &mut self.ants {
            Some(ants) => ants.extend(new_ants),
            None => {}
        }
        match &mut self.food {
            Some(food) => food.extend(new_food),
            None => {}
        }
        match &self.commands {
            Some(commands) => commands.clone(),
            None => Vec::new(),
        }
    }
}

#[test]
fn should_have_turn_x_state_as_name() {
    let mut strategy = TestStrategy::new(None, None, None);
    let state = TurnXState::new(&mut strategy);
    assert_eq!(state.name(), "turn_x_state");
}

#[test]
fn should_read_until_go() {
    let input = b"\
go

turn 2
1";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new(None, None, None);
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert_eq!(reader.bytes().count(), 1);
}

#[test]
fn should_parse_ants_and_pass_them_to_strategy() {
    let input = b"\
a 1 2 3
a 4 5 6
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut ants = Vec::new();
    let mut strategy = TestStrategy::new(Some(&mut ants), None, None);
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert_eq!(ants.len(), 2);
    assert_eq!(ants[0], Ant::new(Location::new(1, 2), 3));
    assert_eq!(ants[1], Ant::new(Location::new(4, 5), 6));
}

#[test]
fn should_parse_food_and_pass_it_to_strategy() {
    let input = b"\
f 1 2
f 3 4
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut food = Vec::new();
    let mut strategy = TestStrategy::new(None, Some(&mut food), None);
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert_eq!(food.len(), 2);
    assert_eq!(food[0], Food::new(Location::new(1, 2)));
    assert_eq!(food[1], Food::new(Location::new(3, 4)));
}

#[test]
fn should_write_commands_and_go() {
    let input = b"\
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let commands = vec![
        Command::new(Location::new(1, 2), Direction::North),
        Command::new(Location::new(3, 4), Direction::East),
        Command::new(Location::new(5, 6), Direction::South),
        Command::new(Location::new(7, 8), Direction::West),
    ];
    let mut strategy = TestStrategy::new(None, None, Some(commands));
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    let output = String::from_utf8(output).unwrap();
    let expected_output = "\
o 1 2 N
o 3 4 E
o 5 6 S
o 7 8 W
go\n";
    assert_eq!(output, expected_output);
}

#[test]
fn should_stay_in_turn_x_state() {
    let input = b"\
go

turn 2
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new(None, None, None);
    let state = TurnXState::new(&mut strategy);
    let next_state = state.parse(&mut reader, &mut output).unwrap();

    assert_eq!(next_state.name(), "turn_x_state");
}

#[test]
fn should_go_to_end_state_on_end() {
    let input = b"\
go

end
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new(None, None, None);
    let state = TurnXState::new(&mut strategy);
    let next_state = state.parse(&mut reader, &mut output).unwrap();

    assert_eq!(next_state.name(), "end_state");
}
