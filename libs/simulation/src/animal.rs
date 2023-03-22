use crate::*;

pub struct Animal {
    pub(crate) pos: Point2<f32>,
    pub(crate) rot: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::Network::random(
            rng,
            &[
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                nn::LayerTopology { neurons: 2 },
            ],
        );

        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.pos
    }

    pub fn rot(&self) -> Rotation2<f32> {
        self.rot
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}
