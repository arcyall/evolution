use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::*;

#[derive(Default)]
pub struct RankSelection;

impl SelectionMethod for RankSelection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual {
        let n = population.len();
        let mut ranks: Vec<f32> = Vec::with_capacity(n);
        let rank_sum: f32 = n as f32 * (n as f32 + 1.0) / 2.0;

        for individual in population {
            let mut rank = 1.0;

            for comp in population {
                if individual.fitness() > comp.fitness() {
                    rank += 1.0
                }
            }

            ranks.push(rank / rank_sum)
        }

        let dist = WeightedIndex::new(ranks).unwrap();

        &population[dist.sample(rng)]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = RankSelection::default();

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
}