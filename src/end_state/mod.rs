#[cfg(test)]
mod test;

use super::state_machine;

use std::io::BufRead;

pub struct EndState {}

impl EndState {
    pub fn new() -> Box<Self> {
        Box::new(EndState {})
    }
}

impl<'a> state_machine::State<'a> for EndState {
    fn name(self: Box<Self>) -> &'a str {
        "end_state"
    }

    fn parse(self: Box<Self>, reader: &mut BufRead) -> Option<Box<state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "go" {
                break;
            }
        }
        None
    }
}
