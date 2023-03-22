pub use self::{animal::*, eye::*, food::*, world::*};
use nalgebra::{distance, wrap, Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};

mod animal;
mod eye;
mod food;
mod world;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movement();
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.pos += animal.rot * Vector2::new(0.0, animal.speed);

            animal.pos.x = wrap(animal.pos.x, 0.0, 1.0);
            animal.pos.y = wrap(animal.pos.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                let dist = distance(&animal.pos, &food.pos);

                if dist <= 0.02 {
                    food.pos = rng.gen();
                }
            }
        }
    }
}
