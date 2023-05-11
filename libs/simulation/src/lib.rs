pub use self::{animal::*, animal_individual::*, brain::*, eye::*, food::*, world::*};
use lib_neural_network as nn;
use nalgebra::{distance, wrap, DVector, Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};
use rayon::prelude::*;
use std::f32::consts::FRAC_PI_2;

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

const GENERATION_LEN: usize = 2500;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.004;
const SPEED_ACCEL: f32 = 0.2;
const ROT_ACCEL: f32 = FRAC_PI_2;

pub struct Simulation {
    world: World,
    ga: nn::GeneticAlgorithm,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
            ga: nn::GeneticAlgorithm::new(
                nn::Selection::Tournament,
                nn::Crossover::Uniform,
                nn::Mutation::Gaussian(0.01, 0.3),
            ),
            age: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<nn::Statistics> {

        self.process_collisions(rng);
        self.world.animals.par_iter_mut().for_each(|animal| {
            animal.process_brain(&self.world.food);
            animal.process_movement();
        });


        self.age += 1;

        if self.age > GENERATION_LEN {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> nn::Statistics {
        loop {
            if let Some(stats) = self.step(rng) {
                return stats;
            }
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                let dist = distance(&animal.pos, &food.pos);

                if dist <= 0.02 {
                    animal.collisions += 1;
                    food.pos = rng.gen();
                }
            }
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> nn::Statistics {
        self.age = 0;

        let current_pop: Vec<AnimalIndividual> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let (evolved_pop, stats) = self.ga.evolve(rng, &current_pop);

        self.world.animals = evolved_pop
            .into_iter()
            .map(|indiv| indiv.into_animal(rng))
            .collect();

        for food in &mut self.world.food {
            food.pos = rng.gen();
        }

        stats
    }
}
