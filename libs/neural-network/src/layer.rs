use crate::neuron::Neuron;

#[derive(Debug)]
pub(crate) struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    #[cfg(test)]
    pub(crate) fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }

    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
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

    pub fn from_weights(
        input_neurons: usize,
        output_neurons: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        Self {
            neurons: (0..output_neurons)
                .map(|_| Neuron::from_weights(input_neurons, weights))
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
        fn test() {
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
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
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
    }
}
