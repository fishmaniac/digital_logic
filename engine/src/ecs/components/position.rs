use crate::ecs::{entities::Entities, events::Event};
use super::super::components::{ComponentStorage, EngineComponent};

impl ComponentStorage for Position {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities.position_components
            .get_mut(entity_id as usize).and_then(|position| position.as_mut())
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        
    }
    fn create(entities: &mut Entities, component: EngineComponent) {
        entities.create_component(component);
        // entities.create_entity()
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self{
        Self {
            x,
            y,
        }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn movement_callback(&mut self, event: &Event) {
        match event {
            Event::StateUpdate => {},
            Event::Position(x, y) => {
                self.x += x;
                self.y += y;
            },
            Event::LeftClick(_, _) => todo!(),
        }
        // println!("Movement listener {}, {}", self.x, self.y);
    }
}
