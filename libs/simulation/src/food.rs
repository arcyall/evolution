use crate::*;

pub struct Food {
    pub(crate) pos: Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self { pos: rng.gen() }
    }

    pub fn position(&self) -> Point2<f32> {
        self.pos
    }
}
