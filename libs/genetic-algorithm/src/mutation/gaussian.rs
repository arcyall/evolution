use crate::*;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f64,
    coeff: f64,
}

impl GaussianMutation {
    pub fn new(chance: f64, coeff: f64) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f64>()
            }
        })
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
mod test {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{GaussianMutation, MutationMethod};

    fn actual(chance: f64, coeff: f64) -> Vec<f64> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }
    mod zero_chance {
        fn actual(coeff: f64) -> Vec<f64> {
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
        fn actual(coeff: f64) -> Vec<f64> {
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
        fn actual(coeff: f64) -> Vec<f64> {
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
