use crate::*;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Mutation {
    /// Mutation with a gaussian distribution.
    /// (chance, coeff)
    Gaussian(f32, f32),
}

impl Mutation {
    pub fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        match self {
            Self::Gaussian(chance, coeff) => self.gaussian(rng, child, *chance, *coeff),
        }
    }

    fn gaussian(&self, rng: &mut dyn RngCore, child: &mut Chromosome, chance: f32, coeff: f32) {
        assert!((0.0..=1.0).contains(&chance));

        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(chance as _) {
                *gene += sign * coeff * rng.gen::<f32>()
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        Mutation::Gaussian(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }
    mod zero_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod zero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_unchanged() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod nonzero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_unchanged() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
    mod fifty_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod zero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_unchanged() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod nonzero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_slightly_changed() {
                let actual = actual(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
    mod max_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod zero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_unchanged() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
        mod nonzero_coefficient {
            use super::*;

            #[test]
            fn original_chromosome_entirely_changed() {
                let actual = actual(1.0);
                let expected = vec![1.9090631, 2.2324157, 2.5512497, 3.901025, 4.2773824];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }
}
