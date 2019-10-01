#[cfg(test)]
mod test;

use super::state_machine;
use super::strategies::Strategy;
use super::turn_x_state::TurnXState;

use std::io::{BufRead, Write};

pub struct Turn0State<'a> {
    strategy: &'a mut dyn Strategy,
}

impl<'a> Turn0State<'a> {
    pub fn new(strategy: &'a mut dyn Strategy) -> Box<Self> {
        Box::new(Turn0State { strategy })
    }
}

impl<'a> state_machine::State<'a> for Turn0State<'a> {
    fn name(self: Box<Self>) -> &'a str {
        "turn_0_state"
    }

    fn parse(
        self: Box<Self>,
        reader: &mut dyn BufRead,
        writer: &mut dyn Write,
    ) -> Option<Box<dyn state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "ready" {
                break;
            }
        }

        writer.write("go\n".as_bytes()).ok();

        Some(TurnXState::new(self.strategy))
    }
}
