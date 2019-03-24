use crate::state_machine::State;
use crate::strategies::{Ant, Command, Food, Strategy};

use super::Turn0State;

use std::io::prelude::*;
use std::io::BufReader;

struct TestStrategy {}

impl TestStrategy {
    fn new() -> Self {
        TestStrategy {}
    }
}

impl Strategy for TestStrategy {
    fn run(&mut self, _ants: &Vec<Ant>, _food: &Vec<Food>) -> Vec<Command> {
        Vec::new()
    }
}

#[test]
fn should_have_turn_0_state_as_name() {
    let mut strategy = TestStrategy::new();
    let state = Turn0State::new(&mut strategy);
    assert_eq!(state.name(), "turn_0_state");
}

#[test]
fn should_read_until_ready() {
    let input = b"\
ready
1";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new();
    let state = Turn0State::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert_eq!(reader.bytes().count(), 1);
}

#[test]
fn should_write_go() {
    let input = b"\
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new();
    let state = Turn0State::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    let output = String::from_utf8(output).unwrap();
    assert_eq!(output, "go\n");
}

#[test]
fn should_go_to_turn_x_state() {
    let input = b"ready";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new();
    let state = Turn0State::new(&mut strategy);

    let next_state = state.parse(&mut reader, &mut output).unwrap();

    assert_eq!(next_state.name(), "turn_x_state");
}
