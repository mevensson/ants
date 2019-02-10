use super::state_machine::State;
use super::Turn0State;

use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn should_have_turn_0_state_as_name() {
    let state = Turn0State::new();
    assert_eq!(state.name(), "turn_0_state");
}

#[test]
fn should_read_until_ready() {
    let input = b"\
ready
1";
    let mut reader = BufReader::new(&input[..]);

    let state = Turn0State::new();

    state.parse(&mut reader);

    assert_eq!(reader.bytes().count(), 1);
}

#[test]
fn should_go_to_turn_x_state() {
    let input = b"ready";
    let mut reader = BufReader::new(&input[..]);

    let state = Turn0State::new();

    let next_state = state.parse(&mut reader).unwrap();

    assert_eq!(next_state.name(), "turn_x_state");
}
