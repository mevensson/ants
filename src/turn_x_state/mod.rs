#[cfg(test)]
mod test;

use super::end_state::EndState;
use super::state_machine;

use std::io::BufRead;

pub struct TurnXState {}

impl TurnXState {
    pub fn new() -> Box<Self> {
        Box::new(TurnXState {})
    }
}

impl<'a> state_machine::State<'a> for TurnXState {
    fn name(self: Box<Self>) -> &'a str {
        "turn_x_state"
    }

    fn parse(self: Box<Self>, reader: &mut BufRead) -> Option<Box<state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "go" {
                break;
            }
        }
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("turn ") {
                return Some(self);
            } else if line == "end" {
                return Some(EndState::new());
            }
        }
        None
    }
}
