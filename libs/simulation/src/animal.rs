use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};

pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rot: Rotation2<f32>,
    pub(crate) speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rot(&self) -> Rotation2<f32> {
        self.rot
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}
