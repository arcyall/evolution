use rand::Rng;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
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

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        Self {
            neurons: (0..output_neurons)
                .map(|_| Neuron::random(rng, input_neurons))
                .collect(),
        }
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        (inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>()
            + self.bias)
            .max(0.0)
    }

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        Self {
            bias: rng.gen_range(-1.0..=1.0),
            weights: (0..output_size)
                .map(|_| rng.gen_range(-1.0..=1.0))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test_neuron() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            );
        }

        #[test]
        fn test_layer() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 4, 3);

            let actual_biases: Vec<f32> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
            let expected_biases = vec![-0.6255188, -0.53516835, -0.19277132];

            let actual_weights: Vec<&[f32]> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();
            let expected_weights: Vec<&[f32]> = vec![
                &[0.67383957, 0.8181262, 0.26284897, 0.5238807],
                &[0.069369674, -0.7648182, -0.102499366, -0.48879617],
                &[-0.8020501, 0.2754606, -0.98680043, 0.4452356],
            ];

            assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
            assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }

        #[test]
        fn test_network() {
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
        fn test_neuron() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
            );
        }

        #[test]
        fn test_layer() {
            let layer = Layer {
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
            };
            let inputs = &[0.5, 0.0, -0.5];

            let actual = layer.propagate(inputs.to_vec());
            let expected = vec![
                layer.neurons[0].propagate(inputs),
                layer.neurons[1].propagate(inputs),
            ];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }

        #[test]
        fn test_network() {
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
            let expected = network.layers[1].propagate(network.layers[0].propagate(inputs.to_vec()));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
