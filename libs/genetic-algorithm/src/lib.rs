#![feature(type_alias_impl_trait)]

use rand::{seq::SliceRandom, Rng, RngCore};
use std::ops::Index;

pub trait Individual {
    fn fitness(&self) -> f32;

    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

#[derive(Default)]
pub struct RouletteSelection;

#[derive(Default)]
pub struct UniformCrossover;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>()
            }
        })
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[allow(clippy::len_without_is_empty)]
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);
                
                todo!()
            })
            .collect()
    }
}

impl SelectionMethod for RouletteSelection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("empty pop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &Chromosome {
            panic!("TestIndividual")
        }
    }

    #[test]
    fn test_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RouletteSelection::default();

        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];

        let actual_histogram: BTreeMap<i32, i32> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as i32).or_default() += 1;

                histogram
            });

        let expected_histogram = BTreeMap::from_iter(vec![(1, 102), (2, 198), (3, 301), (4, 399)]);

        assert_eq!(actual_histogram, expected_histogram);
    }

    #[test]
    fn test_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|x| x as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|x| -x as f32).collect();
        let child = UniformCrossover::default().crossover(&mut rng, &parent_a, &parent_b);
        let diff_a = child
            .clone()
            .into_iter()
            .zip(parent_a)
            .filter(|(c, p)| c != p)
            .count();
        let diff_b = child
            .into_iter()
            .zip(parent_b)
            .filter(|(c, p)| c != p)
            .count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }

    mod test_mutation {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::{GaussianMutation, MutationMethod};

        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }
        mod zero_chance {
            use super::*;

            mod zero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_unchanged() {
                    let actual = actual(0.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
            mod nonzero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_unchanged() {
                    let actual = actual(0.0, 0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
        mod fifty_chance {
            use super::*;

            mod zero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_unchanged() {
                    let actual = actual(0.5, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
            mod nonzero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_slightly_changed() {
                    let actual = actual(0.5, 0.5);
                    let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
        mod max_chance {
            use super::*;

            mod zero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_unchanged() {
                    let actual = actual(1.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
            mod nonzero_coefficient {
                use super::*;

                #[test]
                fn original_chromosome_entirely_changed() {
                    let actual = actual(1.0, 1.0);
                    let expected = vec![1.9090631, 2.2324157, 2.5512497, 3.901025, 4.2773824];

                    approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }
}
