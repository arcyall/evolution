pub use self::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*, statistics::*,
};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;

use crate::*;

pub struct GeneticAlgorithm {
    selection_method: Selection,
    crossover_method: Crossover,
    mutation_method: Mutation,
}

impl GeneticAlgorithm {
    pub fn new(
        selection_method: Selection,
        crossover_method: Crossover,
        mutation_method: Mutation,
    ) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let new_pop = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);

                I::create(child)
            })
            .collect();

        let statistics = Statistics::new(population);

        (new_pop, statistics)
    }
}
