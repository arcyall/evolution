use crate::*;

pub(crate) struct Layer {
    pub(crate) weights: DMatrix<f32>,
    pub(crate) biases: DVector<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Layer {
    pub(crate) fn propagate(&self, input: DVector<f32>) -> DVector<f32> {
        Self::relu(&(&self.weights * input + &self.biases))
    }

    fn relu(input: &DVector<f32>) -> DVector<f32> {
        input.map(|x| x.max(0.0))
    }

    pub(crate) fn random(input_size: usize, output_size: usize, rng: &mut dyn RngCore) -> Self {
        Self {
            weights: DMatrix::from_fn(output_size, input_size, |_, _| rng.gen_range(-1.0..1.0)),
            biases: DVector::from_fn(output_size, |_, _| rng.gen_range(-1.0..1.0)),
        }
    }

    pub(crate) fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let mut biases: Vec<f32> = Vec::with_capacity(output_size);
        let mut weightsvec: Vec<f32> = Vec::with_capacity(output_size * input_size);

        for _ in 0..output_size {
            biases.push(weights.next().unwrap());
        }

        for _ in 0..input_size * output_size {
            weightsvec.push(weights.next().unwrap())
        }

        Self {
            weights: DMatrix::from_vec(output_size, input_size, weightsvec),
            biases: DVector::from_vec(biases),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test() {
        let layer = Layer {
            weights: DMatrix::from_vec(2, 3, vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]),
            biases: DVector::from_vec(vec![0.0, 0.0]),
        };

        let inputs = &[0.5, 0.0, -0.5];

        let actual = layer.propagate(DVector::from_vec(inputs.to_vec()));

        assert_relative_eq!(actual.as_slice(), &[0.0, 0.0].as_slice());
    }
}
