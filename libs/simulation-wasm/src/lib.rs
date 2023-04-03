use lib_simulation as sim;
use rand::{rngs::ThreadRng, thread_rng};
use serde::Serialize;
use wasm_bindgen::prelude::*;
pub use self::{simulation::* , world::* , animal::* , food::* };

mod simulation;
mod world;
mod animal;
mod food;





