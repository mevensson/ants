#[cfg(test)]
mod test;

use std::io::{BufRead, Write};

pub trait State<'a> {
    fn name(self: Box<Self>) -> &'a str {
        ""
    }
    fn parse(
        self: Box<Self>,
        reader: &mut dyn BufRead,
        writer: &mut dyn Write,
    ) -> Option<Box<dyn State<'a> + 'a>>;
}

pub struct StateMachine<'a> {
    current_state: Option<Box<dyn State<'a> + 'a>>,
}

impl<'a> StateMachine<'a> {
    pub fn new(start_state: Box<dyn State<'a> + 'a>) -> Self {
        StateMachine {
            current_state: Some(start_state),
        }
    }

    pub fn parse(&mut self, reader: &mut dyn BufRead, writer: &mut dyn Write) -> bool {
        match self.current_state.take() {
            Some(cs) => {
                let res = cs.parse(reader, writer);
                match res {
                    Some(next_state) => {
                        self.current_state = Some(next_state);
                        false
                    }
                    None => true,
                }
            }
            None => true,
        }
    }
}
