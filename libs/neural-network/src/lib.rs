#![feature(array_windows)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use crate::layer::*;
pub use crate::{geneticalgorithm::*, layer::LayerTopology};
use nalgebra::{DMatrix, DVector};
use rand::{Rng, RngCore};

mod geneticalgorithm;
mod layer;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, input: DVector<f32>) -> DVector<f32> {
        self.layers
            .iter()
            .fold(input, |input, layer| layer.propagate(input))
    }

    pub fn random(layers: &[LayerTopology], rng: &mut dyn RngCore) -> Self {
        assert!(layers.len() > 1);

        Self {
            layers: layers
                .array_windows::<2>()
                .map(|[fst, snd]| Layer::random(fst.neurons, snd.neurons, rng))
                .collect(),
        }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.biases.iter().chain(layer.weights.iter()))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .array_windows::<2>()
            .map(|[fst, snd]| Layer::from_weights(fst.neurons, snd.neurons, &mut weights))
            .collect();

        assert!(weights.next().is_none());

        Self { layers }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let network = Network {
                layers: vec![
                    Layer {
                        weights: DMatrix::from_vec(2, 3, vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]),
                        biases: DVector::from_vec(vec![0.0, 0.0]),
                    },
                    Layer {
                        weights: DMatrix::from_vec(1, 2, vec![0.3, -0.3]),
                        biases: DVector::from_vec(vec![0.0]),
                    },
                ],
            };

            let inputs = DVector::from_vec(vec![0.1, 0.2, 0.3]);

            let actual = network.propagate(inputs.clone());
            let expected = network.layers[1].propagate(network.layers[0].propagate(inputs));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network {
                layers: vec![
                    Layer {
                        weights: DMatrix::from_vec(2, 2, vec![0.2, 0.3, 0.4, 0.5]),
                        biases: DVector::from_vec(vec![0.1]),
                    },
                    Layer {
                        weights: DMatrix::from_vec(2, 2, vec![0.7, 0.8, 0.9, 1.0]),
                        biases: DVector::from_vec(vec![0.6]),
                    },
                ],
            };

            let actual: Vec<f32> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }

        #[test]
        fn test_from_weights() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            assert_relative_eq!(actual.as_slice(), weights.as_slice(),);
        }
    }
}
