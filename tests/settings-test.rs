use ants;
use std::collections::HashMap;

#[macro_use]
extern crate matches;

#[test]
fn should_fail_if_input_does_not_start_with_turn_0() {
    let input = b"";

    let result = ants::Settings::parse(&input[..]);

    assert_matches!(result, Err(_));
}

#[test]
fn should_add_settings_to_values() {
    let input = b"turn 0\nname value";

    let result = ants::Settings::parse(&input[..]);

    let mut expected_values = HashMap::new();
    expected_values.insert("name", "value");

    assert_matches!(
        result,
        Ok(ants::Settings {
            values: expected_values
        })
    );
}

#[test]
fn should_fail_if_input_does_not_end_with_ready() {
    let input = b"turn 0\n";

    let result = ants::Settings::parse(&input[..]);

    assert_matches!(result, Err(_));
}
