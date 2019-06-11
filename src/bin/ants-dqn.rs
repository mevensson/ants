use ants::{self, DqnStrategy, StartState};
use std::io;

fn main() {
    let mut strategy = DqnStrategy::new();
    let start_state = StartState::new(&mut strategy);
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    ants::run(start_state, &mut input, &mut output);
}
