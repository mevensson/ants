use ants::{self, StartState, Strategy};
use std::io;

struct DummyStrategy {}

impl DummyStrategy {
    fn new() -> Self {
        DummyStrategy {}
    }
}

impl Strategy for DummyStrategy {
    fn run(&mut self) {}
}

fn main() {
    let mut strategy = DummyStrategy::new();
    let start_state = StartState::new(&mut strategy);
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    ants::run(start_state, &mut input, &mut output);
}
