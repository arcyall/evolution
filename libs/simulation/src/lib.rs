pub use self::{animal::*, animal_individual::*, brain::*, config::*, eye::*, food::*, world::*};
use lib_neural_network as nn;
use nalgebra::{distance, wrap, DVector, Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};
use rayon::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

mod animal;
mod animal_individual;
mod brain;
mod config;
mod eye;
mod food;
mod world;

pub struct Simulation {
    world: World,
    age: usize,
    config: Config,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, config: Config) -> Self {
        Self {
            world: World::random(rng, &config),
            config,
            age: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<nn::Statistics> {
        self.process_collisions(rng);
        self.world.animals.par_iter_mut().for_each(|animal| {
            animal.process_brain(&self.world.food, &self.config);
            animal.process_movement();
        });

        self.age += 1;

        if self.age > self.config.gen_len {
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

        let ga = nn::GeneticAlgorithm::new(
            self.config.selection_method,
            self.config.crossover_method,
            self.config.mutation_method,
        );

        let (evolved_pop, stats) = ga.evolve(rng, &current_pop);

        self.world.animals = evolved_pop
            .into_iter()
            .map(|indiv| indiv.into_animal(rng, &self.config))
            .collect();

        for food in &mut self.world.food {
            food.pos = rng.gen();
        }

        stats
    }
}
