pub use self::{animal::*, food::*, simulation::*, world::*};
use lib_neural_network as nn;
use lib_simulation as sim;
use rand::{rngs::ThreadRng, thread_rng};
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod animal;
mod food;
mod simulation;
mod world;
