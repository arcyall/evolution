use crate::*;
use strum::IntoEnumIterator;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
    crossovermethods: Vec<&'static str>,
    mutationmethods: Vec<&'static str>,
    selectionmethods: Vec<&'static str>,
}

#[allow(dead_code)]
#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        console_error_panic_hook::set_once();

        let config: sim::Config = serde_wasm_bindgen::from_value(config).unwrap();
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng, config);
        let crossovermethods = nn::geneticalgorithm::Crossover::iter()
            .map(|x| x.into())
            .collect();
        let selectionmethods = nn::geneticalgorithm::Selection::iter()
            .map(|x| x.into())
            .collect();
        let mutationmethods = nn::geneticalgorithm::Mutation::iter()
            .map(|x| x.into())
            .collect();

        Self {
            rng,
            sim,
            crossovermethods,
            selectionmethods,
            mutationmethods,
        }
    }

    pub fn default_config() -> JsValue {
        serde_wasm_bindgen::to_value(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self.sim.config()).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn crossover_methods(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.crossovermethods).unwrap()
    }

    pub fn selection_methods(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.selectionmethods).unwrap()
    }

    pub fn mutation_methods(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.mutationmethods).unwrap()
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        format!(
            "Min. Fitness = {:.2}</br>Max. Fitness = {:.2}</br>Avg. Fitness = {:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
}
