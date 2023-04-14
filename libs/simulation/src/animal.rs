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
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(&eye, rng);

        Self {
            pos: rng.gen(),
            rot: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            collisions: 0,
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

    pub(crate) fn as_chromosome(&self) -> nn::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: nn::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    // pub(crate) fn process_collision(&mut self, rng: &mut dyn RngCore, food: &mut Vec<Food>) {
    //     for food in food {
    //         let dist = distance(&self.pos, &food.pos);

    //         if dist <= 0.02 {
    //             self.collisions += 1;
    //             food.pos = rng.gen();
    //         }
    //     }
    // }

    // pub(crate) fn process_brain(&mut self, food: &[Food]) {
    //     let vision = self.eye.process_vision(self.pos, self.rot, &food);

    //     let response = self.brain.nn.propagate(vision);
    //     let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
    //     let rot = response[1].clamp(-ROT_ACCEL, ROT_ACCEL);

    //     self.speed = (self.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
    //     self.rot = Rotation2::new(self.rot.angle() + rot);
    // }

    // pub(crate) fn process_movement(&mut self) {
    //     self.pos += self.rot * Vector2::new(0.0, self.speed);

    //     self.pos.x = wrap(self.pos.x, 0.0, 1.0);
    //     self.pos.y = wrap(self.pos.y, 0.0, 1.0);
    // }

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
