use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub eye_fov: f32,
    pub eye_range: f32,
    pub eye_cells: usize,

    pub brain_neurons: usize,

    pub speed_min: f32,
    pub speed_max: f32,
    pub speed_accel: f32,
    pub rot_accel: f32,

    pub gen_len: usize,

    pub count_animal: usize,
    pub count_food: usize,

    pub selection_method: nn::Selection,
    pub mutation_method: nn::Mutation,
    pub crossover_method: nn::Crossover,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eye_fov: PI + FRAC_PI_2,
            eye_range: 0.25,
            eye_cells: 9,
            brain_neurons: 9,
            speed_min: 0.002,
            speed_max: 0.6,
            speed_accel: 0.2,
            rot_accel: FRAC_PI_2,
            gen_len: 3000,
            count_animal: 30,
            count_food: 100,
            selection_method: nn::Selection::Roulette,
            mutation_method: nn::Mutation::Gaussian(0.01, 0.3),
            crossover_method: nn::Crossover::Uniform,
        }
    }
}
