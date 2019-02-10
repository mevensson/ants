use super::state_machine::State;
use super::EndState;

use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn should_have_end_state_as_name() {
    let state = EndState::new();
    assert_eq!(state.name(), "end_state");
}

#[test]
fn should_read_until_go() {
    let input = b"\
go
1";
    let mut reader = BufReader::new(&input[..]);

    let state = EndState::new();

    state.parse(&mut reader);

    assert_eq!(reader.bytes().count(), 1);
}

#[test]
fn should_end() {
    let input = b"ready";
    let mut reader = BufReader::new(&input[..]);

    let state = EndState::new();

    let next_state = state.parse(&mut reader);

    assert!(next_state.is_none());
}
