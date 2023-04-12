#![feature(array_windows)]

use crate::layer::*;
pub use crate::layertopology::*;
use nalgebra::{DMatrix, DVector};

mod layer;
mod layertopology;
mod neuron;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    #[cfg(test)]
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propagate(&self, input: DVector<f64>) -> DVector<f64> {
        self.layers
            .iter()
            .fold(input, |input, layer| layer.propagate(input))
    }

    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        Self {
            layers: layers
                .array_windows::<2>()
                .map(|[fst, snd]| Layer::random(rng, fst.neurons, snd.neurons))
                .collect(),
        }
    }

    pub fn weights(&self) -> impl Iterator<Item = f64> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.weights.iter().chain(layer.biases.iter()))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f64>) -> Self {
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
    use crate::neuron::Neuron;
    use approx::assert_relative_eq;

    mod random {
        use super::*;
        use crate::layertopology::LayerTopology;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 4 },
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ],
            );

            assert_eq!(network.layers.len(), 3);
            assert_eq!(network.layers[0].neurons.len(), 3);

            assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
            assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_slice()
            );

            assert_relative_eq!(network.layers[0].neurons[1].bias, -0.53516835);
            assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                &[0.069369674, -0.7648182, -0.102499366, -0.48879617].as_slice()
            );

            assert_relative_eq!(network.layers[0].neurons[2].bias, -0.19277132);
            assert_relative_eq!(
                network.layers[0].neurons[2].weights.as_slice(),
                &[-0.8020501, 0.2754606, -0.98680043, 0.4452356].as_slice()
            );

            assert_eq!(network.layers[1].neurons.len(), 2);

            assert_relative_eq!(network.layers[1].neurons[0].bias, -0.47662205);
            assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.89078736, -0.361278, -0.14956534].as_slice()
            );

            assert_relative_eq!(network.layers[1].neurons[1].bias, 0.35662687);
            assert_relative_eq!(
                network.layers[1].neurons[1].weights.as_slice(),
                &[-0.8566594, 0.33309853, 0.11767423].as_slice()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![
                            Neuron {
                                bias: 0.0,
                                weights: vec![0.1, 0.2, 0.3],
                            },
                            Neuron {
                                bias: 0.0,
                                weights: vec![0.4, 0.5, 0.6],
                            },
                        ],
                    },
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.0,
                            weights: vec![0.3, -0.3],
                        }],
                    },
                ],
            };
            let inputs: &[f32] = &[0.1, 0.2, 0.3];

            let actual = network.propagate(inputs.to_vec());
            let expected =
                network.layers[1].propagate(network.layers[0].propagate(inputs.to_vec()));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual: Vec<f32> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
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
