use crate::*;

pub trait Individual {
    fn fitness(&self) -> f64;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
#[derive(PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f64 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f64) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("unsupported for testindividual"),
        }
    }

    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn fitness(&self) -> f64 {
        match self {
            TestIndividual::WithChromosome { chromosome } => chromosome.iter().sum(),
            TestIndividual::WithFitness { fitness } => *fitness,
        }
    }
}
