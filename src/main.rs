use ants::{self, StartState};
use std::io;

fn main() {
    let start_state = StartState::new();
    let stdin = io::stdin();
    let mut input = stdin.lock();
    ants::run(start_state, &mut input);
}
