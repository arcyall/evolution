use crate::*;

#[derive(Serialize)]
pub struct Animal {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
    pub speed: f64,
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