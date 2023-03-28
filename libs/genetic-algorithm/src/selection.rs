pub use self::{roulette::*, rank::*,};
use crate::*;

mod roulette;
mod rank;

pub trait SelectionMethod {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual;
}
