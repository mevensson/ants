use crate::state_machine::State;
use crate::strategies::{Ant, Strategy};

use super::TurnXState;

use std::io::prelude::*;
use std::io::BufReader;

struct TestStrategy<'a> {
    ants: Option<&'a mut Vec<Ant>>,
}

impl<'a> TestStrategy<'a> {
    fn new(ants: Option<&'a mut Vec<Ant>>) -> Self {
        TestStrategy { ants }
    }
}

impl<'a> Strategy for TestStrategy<'a> {
    fn run(&mut self, mut new_ants: Vec<Ant>) {
        match &mut self.ants {
            Some(ants) => ants.append(&mut new_ants),
            None => {}
        }
    }
}

#[test]
fn should_have_turn_x_state_as_name() {
    let mut strategy = TestStrategy::new(None);
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

    let mut strategy = TestStrategy::new(None);
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
    let mut strategy = TestStrategy::new(Some(&mut ants));
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert_eq!(ants.len(), 2);
    assert_eq!(ants[0], Ant::new(1, 2, 3));
    assert_eq!(ants[1], Ant::new(4, 5, 6));
}

#[test]
fn should_write_go() {
    let input = b"\
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new(None);
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    let output = String::from_utf8(output).unwrap();
    assert_eq!(output, "go\n");
}

#[test]
fn should_stay_in_turn_x_state() {
    let input = b"\
go

turn 2
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new(None);
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

    let mut strategy = TestStrategy::new(None);
    let state = TurnXState::new(&mut strategy);
    let next_state = state.parse(&mut reader, &mut output).unwrap();

    assert_eq!(next_state.name(), "end_state");
}
