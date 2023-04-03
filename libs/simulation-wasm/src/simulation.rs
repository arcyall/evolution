use crate::*;

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

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
}