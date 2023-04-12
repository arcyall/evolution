use crate::*;

pub struct AnimalIndividual {
    fitness: f64,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn fitness(&self) -> f64 {
        self.fitness
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.collisions as f64,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
