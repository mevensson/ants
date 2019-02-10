use super::run;
use crate::state_machine;
use std::io::{BufRead, Write};

struct TestState<'a> {
    num_calls: &'a mut i32,
}

impl<'a> TestState<'a> {
    fn new(num_calls: &'a mut i32) -> Box<Self> {
        Box::new(TestState { num_calls })
    }
}

impl<'a> state_machine::State<'a> for TestState<'a> {
    fn parse(
        self: Box<Self>,
        _reader: &mut BufRead,
        _writer: &mut Write,
    ) -> Option<Box<state_machine::State<'a> + 'a>> {
        *self.num_calls -= 1;
        if *self.num_calls == 0 {
            None
        } else {
            Some(self)
        }
    }
}

#[test]
fn should_run_state_machine_until_done() {
    let mut num_calls = 3;
    {
        let start_state = TestState::new(&mut num_calls);
        let mut input = "".as_bytes();
        let mut output = Vec::new();

        run(start_state, &mut input, &mut output);
    }
    assert_eq!(num_calls, 0);
}
