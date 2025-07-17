use sdl3::pixels::Color;

use super::{entity::Entity, events::Event, state::StateStorage};
use crate::ecs::{
    components::{position::Position, rect::Rect},
    entities::Entities
};

pub mod position;
pub mod rect;

// TODO: move to renderer
pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}
impl From<ColorRGB> for Color {
    fn from(rgb: ColorRGB) -> Self {
        Color::RGB(rgb.r, rgb.g, rgb.b)
    }
}

pub trait ComponentStorage {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self>
where
        Self: Sized;
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event);
    fn create(entities: &mut Entities, component: EngineComponent);
}

pub enum ComponentType {
    Engine(EngineComponent),
    Game(Box<dyn GameComponent>),
}

#[derive(Debug)]
pub enum EngineComponent {
    Position(Position),
    Rect(Rect),
    Draggable,
    State(Box<dyn StateStorage>)
}

pub trait GameComponent {
    fn update(&mut self, entity: &Entity);
}

pub struct Components {
    pub game_components: Vec<Box<dyn GameComponent>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            game_components: Vec::new(),
        }
    }
}
