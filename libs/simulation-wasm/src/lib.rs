use lib_simulation as sim;
use rand::{rngs::ThreadRng, thread_rng};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[allow(dead_code)]
#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) {
        self.sim.train(&mut self.rng);
    }
}

#[derive(Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        Self {
            animals: world.animals().iter().map(Animal::from).collect(),
            food: world.food().iter().map(Food::from).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    pub speed: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rot: animal.rot().angle(),
            speed: animal.speed(),
        }
    }
}

#[derive(Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
