use super::state_machine::State;
use super::Turn0State;

use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn should_read_until_ready() {
    let input = b"\
ready
1";
    let mut reader = BufReader::new(&input[..]);

    let state = Turn0State::new();

    state.parse(&mut reader);

    assert_eq!(reader.bytes().count(), 1);
}
