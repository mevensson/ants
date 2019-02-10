use ants::{self, StartState};

use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn should_read_turn_0() {
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
";
    let mut reader = BufReader::new(&input[..]);

    let start_state = StartState::new();
    ants::run(start_state, &mut reader);

    assert_eq!(reader.bytes().count(), 0);
}

#[test]
fn should_read_turn_1() {
    let input = b"\
turn 0
ready

turn 1
f 6 5
w 7 6
a 7 9 1
a 10 8 0
a 10 9 0
h 7 12 1
go
";
    let mut reader = BufReader::new(&input[..]);

    let start_state = StartState::new();
    ants::run(start_state, &mut reader);

    assert_eq!(reader.bytes().count(), 0);
}

#[test]
fn should_read_end() {
    let input = b"\
turn 0
ready

turn 1
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

    let start_state = StartState::new();
    ants::run(start_state, &mut reader);

    assert_eq!(reader.bytes().count(), 0);
}
