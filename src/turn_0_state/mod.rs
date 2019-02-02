#[cfg(test)]
mod test;

use super::state_machine;

use std::io::BufRead;

pub struct Turn0State {}

impl Turn0State {
    pub fn new() -> Box<Self> {
        Box::new(Turn0State {})
    }
}

impl<'a> state_machine::State<'a> for Turn0State {
    fn parse(self: Box<Self>, reader: &mut BufRead) -> Option<Box<state_machine::State<'a> + 'a>> {
        for line in reader.lines() {
            if line.unwrap() == "ready" {
                break;
            }
        }
        None
    }
}
