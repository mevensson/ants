use super::{Ant, Food, Location};

#[test]
fn should_parse_ant() {
    let ant_string = "1 2 3";

    let ant = Ant::parse(ant_string);

    let expected_ant = Ant::new(Location::new(1, 2), 3);
    assert_eq!(ant, expected_ant);
}

#[test]
fn should_parse_food() {
    let food_string = "1 2";

    let food = Food::parse(food_string);

    let expected_food = Food::new(Location::new(1, 2));
    assert_eq!(food, expected_food);
}
