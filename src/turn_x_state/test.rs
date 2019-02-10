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
fn should_read_until_go() {
    let input = b"\
go

turn 2
1";
    let mut reader = BufReader::new(&input[..]);

    let state = TurnXState::new();

    state.parse(&mut reader);

    assert_eq!(reader.bytes().count(), 1);
}

#[test]
fn should_stay_in_turn_x_state() {
    let input = b"\
go

turn 2
";
    let mut reader = BufReader::new(&input[..]);

    let state = TurnXState::new();
    let next_state = state.parse(&mut reader).unwrap();

    assert_eq!(next_state.name(), "turn_x_state");
}

#[test]
fn should_go_to_end_state_on_end() {
    let input = b"\
go

end
";
    let mut reader = BufReader::new(&input[..]);

    let state = TurnXState::new();
    let next_state = state.parse(&mut reader).unwrap();

    assert_eq!(next_state.name(), "end_state");
}
