use crate::*;

pub struct Animal {
    pub(crate) pos: Point2<f64>,
    pub(crate) rot: Rotation2<f64>,
    pub(crate) speed: f64,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) collisions: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            collisions: 0,
        }
    }

    pub fn position(&self) -> Point2<f64> {
        self.pos
    }

    pub fn rot(&self) -> Rotation2<f64> {
        self.rot
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            collisions: 0,
        }
    }
}
