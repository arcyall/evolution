use crate::*;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct RouletteSelection;

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
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
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
}
