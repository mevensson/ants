mod state_machine;

pub use crate::state_machine::State;
pub use crate::state_machine::StateMachine;

use std::io;

#[cfg(test)]
mod test;

pub fn run<'a>(start_state: Box<State<'a> + 'a>, reader: &mut io::BufRead) {
    let mut state_machine = StateMachine::new(start_state);
    while !state_machine.parse(reader) {}
}
