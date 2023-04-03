use crate::*;

#[derive(Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        Self {
            animals: world.animals().iter().map(Animal::from).collect(),
            food: world.food().iter().map(Food::from).collect(),
        }
    }
}