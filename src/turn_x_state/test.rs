use super::state_machine::State;
use super::TurnXState;

use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn should_have_turn_x_state_as_name() {
    let state = TurnXState::new();
    assert_eq!(state.name(), "turn_x_state");
}

#[test]
fn should_end() {
    let input = b"";
    let mut reader = BufReader::new(&input[..]);

    let state = TurnXState::new();
    let next_state = state.parse(&mut reader);

    assert!(next_state.is_none());
}