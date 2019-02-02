#[cfg(test)]
mod test;

use std::io;

pub trait State<'a> {
    fn parse(self: Box<Self>, reader: &mut io::BufRead) -> Option<Box<State<'a> + 'a>>;
}

pub struct StateMachine<'a> {
    current_state: Option<Box<State<'a> + 'a>>,
}

impl<'a> StateMachine<'a> {
    pub fn new(start_state: Box<State<'a> + 'a>) -> Self {
        StateMachine {
            current_state: Some(start_state),
        }
    }

    pub fn parse(&mut self, reader: &mut io::BufRead) -> bool {
        match self.current_state.take() {
            Some(cs) => {
                let res = cs.parse(reader);
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
