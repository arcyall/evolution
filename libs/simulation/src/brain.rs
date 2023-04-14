use crate::*;

pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(eye: &Eye, rng: &mut dyn RngCore) -> Self {
        Self {
            nn: nn::Network::random(&Self::topology(eye), rng),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: eye.cells() * 2,
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
