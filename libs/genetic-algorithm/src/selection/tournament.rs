use crate::*;
use rand::Rng;

#[derive(Default)]
pub struct TournamentSelection;

impl SelectionMethod for TournamentSelection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
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
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let method = TournamentSelection::default();

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
