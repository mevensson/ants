#[cfg(test)]
mod test;

use super::{Ant, Command, Food, Location, Strategy};

pub struct HuntFoodStrategy {}

impl HuntFoodStrategy {
    pub fn new() -> Self {
        HuntFoodStrategy {}
    }

    fn closest_food(ant: &Ant, food: &Vec<Food>) -> Option<Location> {
        let mut closest_distance_sq = std::i32::MAX;
        let mut closest_location = None;
        for f in food {
            let distance_sq = Location::distance_sq(&ant.location, &f.location);
            if closest_distance_sq > distance_sq {
                closest_distance_sq = distance_sq;
                closest_location = Some(f.location);
            }
        }
        return closest_location;
    }
}

impl Strategy for HuntFoodStrategy {
    fn run(&mut self, ants: &Vec<Ant>, food: &Vec<Food>) -> Vec<Command> {
        let mut result = Vec::new();
        if !food.is_empty() {
            for ant in ants {
                let closest_food_location = HuntFoodStrategy::closest_food(&ant, food);
                match closest_food_location {
                    Some(location) => {
                        let direction = Location::closest_direction(&ant.location, &location);
                        result.push(Command::new(ant.location, direction));
                    }
                    None => {}
                }
            }
        }
        result
    }
}
