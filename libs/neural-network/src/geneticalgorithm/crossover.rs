use crate::*;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(IntoStaticStr, EnumIter, Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Crossover {
    Uniform,
}

impl Crossover {
    pub fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        match self {
            Self::Uniform => self.uniform(rng, parent_a, parent_b),
        }
    }

    fn uniform(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_uniform() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (1..=100).map(|x| x as f32).collect();
        let parent_b: Chromosome = (1..=100).map(|x| -x as f32).collect();
        let child = Crossover::Uniform.crossover(&mut rng, &parent_a, &parent_b);
        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
