use super::{Ant, Direction, Food, Location};

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

#[test]
fn should_calculate_distance_sq() {
    let row = 12;
    let col = 34;

    assert_eq!(
        Location::distance_sq(&Location::new(row, col), &Location::new(row + 5, col + 10)),
        5 * 5 + 10 * 10
    );
    assert_eq!(
        Location::distance_sq(&Location::new(row, col), &Location::new(row - 5, col - 10)),
        5 * 5 + 10 * 10
    );
}

#[test]
fn should_calculate_closest_direction() {
    let row = 12;
    let col = 34;

    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row - 5, col - 5)),
        Direction::North
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row - 5, col + 5)),
        Direction::North
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row - 4, col + 5)),
        Direction::East
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row + 4, col + 5)),
        Direction::East
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row + 5, col - 5)),
        Direction::South
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row + 5, col + 5)),
        Direction::South
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row - 4, col - 5)),
        Direction::West
    );
    assert_eq!(
        Location::closest_direction(&Location::new(row, col), &Location::new(row + 4, col - 5)),
        Direction::West
    );
}
