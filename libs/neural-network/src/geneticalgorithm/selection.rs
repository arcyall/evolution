use crate::*;
use rand::{distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom};

pub enum Selection {
    Rank,
    Roulette,
    Tournament,
}

impl Selection {
    pub fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        match self {
            Self::Rank => self.rank_select(rng, population),
            Self::Roulette => self.roulette_select(rng, population),
            Self::Tournament => self.tournament_select(rng, population),
        }
    }

    fn rank_select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
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

    fn roulette_select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("empty pop")
    }

    fn tournament_select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual,
    {
        let n = population.len();
        let tournament_size = rng.gen_range(0..n);
        let mut selected = &population[rng.gen_range(0..n)];
        let mut best = 0.0;

        for _ in 0..tournament_size {
            let p = &population[rng.gen_range(0..n)];

            if p.fitness() >= best {
                best = p.fitness();
                selected = p;
            }
        }

        selected
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test_rank() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = Selection::Rank;

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
    fn test_roulette() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = Selection::Roulette;

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
    fn test_tournament() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = Selection::Tournament;

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

        let expected_histogram = BTreeMap::from_iter(vec![(1, 142), (2, 215), (3, 275), (4, 368)]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
