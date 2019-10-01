use super::State;
use super::StateMachine;
use std::io::{BufRead, Write};

#[derive(Clone)]
struct TestState {
    next_state: Option<Box<TestState>>,
}

impl TestState {
    fn new(next_state: Option<Box<TestState>>) -> Box<Self> {
        Box::new(TestState { next_state })
    }
}

impl<'a> State<'a> for TestState {
    fn parse(
        self: Box<Self>,
        _reader: &mut dyn BufRead,
        _writer: &mut dyn Write,
    ) -> Option<Box<dyn State<'a> + 'a>> {
        match &self.next_state {
            Some(next_state) => Some(next_state.clone()),
            None => None,
        }
    }
}

#[test]
fn should_end_on_none_state() {
    let start_state = TestState::new(None);
    let mut state_machine = StateMachine::new(start_state);

    let mut input = "".as_bytes();
    let mut output = Vec::new();
    let done = state_machine.parse(&mut input, &mut output);

    assert_eq!(done, true);
}

#[test]
fn should_not_end_on_some_state() {
    let next_state = TestState::new(None);
    let start_state = TestState::new(Some(next_state));
    let mut state_machine = StateMachine::new(start_state);

    let mut input = "".as_bytes();
    let mut output = Vec::new();
    let done = state_machine.parse(&mut input, &mut output);

    assert_eq!(done, false);
}

#[test]
fn should_remember_next_state() {
    let next_state = TestState::new(None);
    let start_state = TestState::new(Some(next_state));
    let mut state_machine = StateMachine::new(start_state);

    let mut input = "".as_bytes();
    let mut output = Vec::new();
    state_machine.parse(&mut input, &mut output);
    let done = state_machine.parse(&mut input, &mut output);

    assert_eq!(done, true);
}
