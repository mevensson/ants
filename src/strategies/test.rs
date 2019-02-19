use super::Ant;

#[test]
fn should_parse_ant() {
    let ant_string = "1 2 3";

    let ant = Ant::parse(ant_string);

    let expected_ant = Ant::new(1, 2, 3);
    assert_eq!(ant, expected_ant);
}
