use crate::layer::*;
pub use crate::layertopology::*;

mod layer;
mod layertopology;
mod neuron;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        Self {
            layers: layers
                .windows(2)
                .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;
    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::layertopology::LayerTopology;

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
        use crate::neuron::Neuron;

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
}
