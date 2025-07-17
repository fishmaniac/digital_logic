use super::components::{EngineComponent, rect::Rect};

pub trait GameEntity {
}

pub struct Entity {
    pub id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self {
            id,
        }
    }
}
