use nalgebra::{Point2, Vector2, Rotation2, wrap};
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

pub struct Animal {
    position: Point2<f32>,
    rot: Rotation2<f32>,
    speed: f32,
}

pub struct Food {
    position: Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rot * Vector2::new(0.0, animal.speed);

            animal.position.x = wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = wrap(animal.position.y, 0.0, 1.0);
        }
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            animals: (0..40).map(|_| Animal::random(rng)).collect(),
            food: (0..60).map(|_| Food::random(rng)).collect(),
        }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
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

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
