use std::{any::Any, fmt};

use sdl3::pixels::Color;

use super::events::Event;
use crate::ecs::{
    components::{
        draggable::Draggable, line::Line, position::Position, rect::Rect, state::StateStorage,
        texture_rect::TextureRect,
    },
    entities::Entities,
    events::{self, EntityEvent, Events},
};

pub mod draggable;
pub mod line;
pub mod position;
pub mod rect;
pub mod state;
pub mod texture_rect;

// TODO: move to renderer
pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
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
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut events::EntityEvents,
        entity_id: u32,
        event: &Event,
    );
    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent);
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events);
}

#[derive(Debug)]
pub enum Component {
    Engine(EngineComponent),
    Game(Box<dyn GameComponent>),
}

#[derive(Debug)]
pub enum EngineComponent {
    Position(Position),
    Rect(Rect),
    Line(Line),
    Texture(TextureRect),
    Draggable(Draggable),
    State(Box<dyn StateStorage>),
}

pub trait GameComponent: fmt::Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
