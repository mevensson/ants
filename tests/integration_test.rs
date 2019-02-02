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
ready";
    let mut reader = BufReader::new(&input[..]);

    let start_state = StartState::new();
    ants::run(start_state, &mut reader);

    assert_eq!(reader.bytes().count(), 0);
}
