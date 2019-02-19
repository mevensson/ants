use super::super::{Ant, Food, Strategy};
use super::HuntFoodStrategy;

#[test]
fn should_direct_ants_to_closest_food() {
    let ants = vec![Ant::new(1, 2, 3)];
    let food = vec![Food::new(4, 5)];

    let mut strategy = HuntFoodStrategy::new();
    strategy.run(ants, food);
}
