use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: nn::Chromosome,
}

impl nn::Individual for AnimalIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &nn::Chromosome {
        &self.chromosome
    }

    fn create(chromosome: nn::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.collisions as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore, config: &Config) -> Animal {
        Animal::from_chromosome(self.chromosome, rng, config)
    }
}
