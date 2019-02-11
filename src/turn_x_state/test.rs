use crate::state_machine::State;
use crate::strategies::Strategy;

use super::TurnXState;

use std::io::prelude::*;
use std::io::BufReader;

struct TestStrategy<'a> {
    run_called: Option<&'a mut bool>,
}

impl<'a> TestStrategy<'a> {
    fn new(run_called: Option<&'a mut bool>) -> Self {
        TestStrategy { run_called }
    }
}

impl<'a> Strategy for TestStrategy<'a> {
    fn run(&mut self) {
        match &mut self.run_called {
            Some(run_called) => **run_called = true,
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
fn should_run_strategy() {
    let input = b"\
ready
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut run_called = false;
    let mut strategy = TestStrategy::new(Some(&mut run_called));
    let state = TurnXState::new(&mut strategy);

    state.parse(&mut reader, &mut output);

    assert!(run_called);
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
