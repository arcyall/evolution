use crate::*;

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
