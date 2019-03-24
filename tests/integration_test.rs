use ants::{self, Ant, Command, Direction, Food, Location, StartState, Strategy};

use std::io::prelude::*;
use std::io::BufReader;

struct TestStrategy {}

impl TestStrategy {
    fn new() -> Self {
        TestStrategy {}
    }
}

impl Strategy for TestStrategy {
    fn run(&mut self, _ants: &Vec<Ant>, _food: &Vec<Food>) -> Vec<Command> {
        vec![
            Command::new(Location::new(10, 8), Direction::North),
            Command::new(Location::new(10, 9), Direction::North),
        ]
    }
}

#[test]
fn should_handle_sample_input() {
    let input = b"\
turn 0
loadtime 3000
turntime 1000
rows 20
cols 20
turns 500
viewradius2 55
attackradius2 5
spawnradius2 1
player_seed 42
ready

turn 1
f 6 5
w 7 6
a 7 9 1
a 10 8 0
a 10 9 0
h 7 12 1
go

end
players 2
score 1 0
f 6 5
d 7 8 1
a 9 8 0
a 9 9 0
go
";
    let mut reader = BufReader::new(&input[..]);
    let mut output = Vec::new();

    let mut strategy = TestStrategy::new();
    let start_state = StartState::new(&mut strategy);
    ants::run(start_state, &mut reader, &mut output);

    assert_eq!(reader.bytes().count(), 0);

    let output = String::from_utf8(output).unwrap();
    let expected_output = "\
go
o 10 8 N
o 10 9 N
go
";
    assert_eq!(output, expected_output);
}
