use crate::*;

pub struct Animal {
    pub(crate) pos: Point2<f32>,
    pub(crate) rot: Rotation2<f32>,
    pub(crate) speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
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
