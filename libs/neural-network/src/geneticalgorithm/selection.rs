pub use self::{rank::*, roulette::*, tournament::*};
use crate::*;

mod rank;
mod roulette;
mod tournament;

pub trait SelectionMethod {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> &'a T
    where
        T: Individual;
}
