mod state_machine;
mod turn_0_state;

pub use crate::turn_0_state::Turn0State as StartState;

use crate::state_machine::State;
use crate::state_machine::StateMachine;

use std::io::BufRead;

#[cfg(test)]
mod test;

pub fn run<'a>(start_state: Box<State<'a> + 'a>, reader: &mut BufRead) {
    let mut state_machine = StateMachine::new(start_state);
    while !state_machine.parse(reader) {}
}
