#[cfg(test)]
mod test;

use super::end_state::EndState;
use super::state_machine;
use super::strategies::Strategy;

use std::io::{BufRead, Write};

pub struct TurnXState<'a> {
    strategy: &'a mut Strategy,
}

impl<'a> TurnXState<'a> {
    pub fn new(strategy: &'a mut Strategy) -> Box<Self> {
        Box::new(TurnXState { strategy })
    }
}

impl<'a> state_machine::State<'a> for TurnXState<'a> {
    fn name(self: Box<Self>) -> &'a str {
        "turn_x_state"
    }

    fn parse(
        self: Box<Self>,
        reader: &mut BufRead,
        writer: &mut Write,
    ) -> Option<Box<state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "go" {
                break;
            }
        }

        self.strategy.run();
        writer.write("go\n".as_bytes()).ok();

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
