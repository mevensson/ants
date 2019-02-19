mod end_state;
mod state_machine;
mod strategies;
mod turn_0_state;
mod turn_x_state;

pub use crate::strategies::{Ant, Food, Strategy};
pub use crate::strategies::hunt_food::HuntFoodStrategy;
pub use crate::turn_0_state::Turn0State as StartState;

use crate::state_machine::State;
use crate::state_machine::StateMachine;

use std::io::{BufRead, Write};

#[cfg(test)]
mod test;

pub fn run<'a>(start_state: Box<State<'a> + 'a>, reader: &mut BufRead, writer: &mut Write) {
    let mut state_machine = StateMachine::new(start_state);
    while !state_machine.parse(reader, writer) {}
}
