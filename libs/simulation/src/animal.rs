use crate::*;

pub struct Animal {
    pub(crate) pos: Point2<f32>,
    pub(crate) rot: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) collisions: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Self {
        let brain = Brain::random(config, rng);

        Self::new(config, brain, rng)
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

    pub(crate) fn as_chromosome(&self) -> nn::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: nn::Chromosome, rng: &mut dyn RngCore, config: &Config) -> Self {
        let brain = Brain::from_chromosome(chromosome, config);

        Self::new(config, brain, rng)
    }

    pub(crate) fn process_brain(&mut self, food: &[Food], config: &Config) {
        let vision = self.eye.process_vision(self.pos, self.rot, food);

        let response = self.brain.nn.propagate(vision);
        let speed = response[0].clamp(-config.speed_accel, config.speed_accel);
        let rot = response[1].clamp(-config.rot_accel, config.rot_accel);

        self.speed = (self.speed + speed).clamp(config.speed_min, config.speed_min);
        self.rot = Rotation2::new(self.rot.angle() + rot);
    }

    pub(crate) fn process_movement(&mut self) {
        self.pos += self.rot * Vector2::new(0.0, self.speed);

        self.pos.x = wrap(self.pos.x, 0.0, 1.0);
        self.pos.y = wrap(self.pos.y, 0.0, 1.0);
    }

    fn new(config: &Config, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: config.speed_max,
            eye: Eye::new(config),
            brain,
            collisions: 0,
        }
    }
}
