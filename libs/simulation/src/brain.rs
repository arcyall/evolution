use crate::*;

pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        Self {
            nn: nn::Network::random(&Self::topology(config), rng),
        }
    }

    pub(crate) fn as_chromosome(&self) -> nn::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: nn::Chromosome, config: &Config) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(config), chromosome),
        }
    }

    fn topology(config: &Config) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: config.eye_cells,
            },
            nn::LayerTopology {
                neurons: config.brain_neurons,
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
