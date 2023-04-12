use crate::*;

#[derive(Serialize)]
pub struct Food {
    pub x: f64,
    pub y: f64,
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}