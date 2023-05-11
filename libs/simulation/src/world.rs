use crate::*;

pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) food: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Self {
        Self {
            animals: (0..config.count_animal).map(|_| Animal::random(rng, config)).collect(),
            food: (0..config.count_food).map(|_| Food::random(rng)).collect(),
        }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
}
