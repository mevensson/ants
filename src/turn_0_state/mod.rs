#[cfg(test)]
mod test;

use super::state_machine;
use super::turn_x_state::TurnXState;

use std::io::{BufRead, Write};

pub struct Turn0State {}

impl Turn0State {
    pub fn new() -> Box<Self> {
        Box::new(Turn0State {})
    }
}

impl<'a> state_machine::State<'a> for Turn0State {
    fn name(self: Box<Self>) -> &'a str {
        "turn_0_state"
    }

    fn parse(
        self: Box<Self>,
        reader: &mut BufRead,
        writer: &mut Write,
    ) -> Option<Box<state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "ready" {
                break;
            }
        }

        writer.write("go\n".as_bytes()).ok();

        Some(TurnXState::new())
    }
}
