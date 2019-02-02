use ants;
use std::io;

struct DummyState {}

impl<'a> ants::State<'a> for DummyState {
    fn parse(self: Box<Self>, _reader: &mut io::BufRead) -> Option<Box<dyn ants::State<'a> + 'a>> {
        None
    }
}

fn main() {
    let start_state = Box::new(DummyState {});
    let stdin = io::stdin();
    let mut input = stdin.lock();
    ants::run(start_state, &mut input);
}
