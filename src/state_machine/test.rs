use super::State;
use super::StateMachine;
use std::io;

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
    fn parse(self: Box<Self>, _reader: &mut io::BufRead) -> Option<Box<State<'a> + 'a>> {
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
    let done = state_machine.parse(&mut input);

    assert_eq!(done, true);
}

#[test]
fn should_not_end_on_some_state() {
    let next_state = TestState::new(None);
    let start_state = TestState::new(Some(next_state));
    let mut state_machine = StateMachine::new(start_state);

    let mut input = "".as_bytes();
    let done = state_machine.parse(&mut input);

    assert_eq!(done, false);
}

#[test]
fn should_remember_next_state() {
    let next_state = TestState::new(None);
    let start_state = TestState::new(Some(next_state));
    let mut state_machine = StateMachine::new(start_state);

    let mut input = "".as_bytes();
    state_machine.parse(&mut input);
    let done = state_machine.parse(&mut input);

    assert_eq!(done, true);
}
