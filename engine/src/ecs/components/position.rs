use sdl3::keyboard::Keycode;

use super::super::components::{ComponentStorage, EngineComponent};
use crate::ecs::{
    entities::Entities,
    events::{EntityEvent, Event, Events},
};

impl ComponentStorage for Position {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities
            .position_components
            .get_mut(entity_id as usize)
            .and_then(|position| position.as_mut())
    }
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut crate::ecs::events::EntityEvents,
        entity_id: u32,
        event: &Event,
    ) {
    }
    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent) {
        match event {
            EntityEvent::Position(_, _) => {
                let mut position = entities.get_component_mut::<Position>(entity_id);
                match position {
                    Some(ref mut position) => position.position_callback(event),
                    None => {}
                }
            }
            _ => {}
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
        entities.create_component(component);
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    // pub fn movement_callback(&mut self, event: &Event) {
    //     match event {
    //         Event::KeyCode(keycode) => match keycode {
    //             Keycode::W => self.y -= 1,
    //             Keycode::A => self.x -= 1,
    //             Keycode::S => self.y += 1,
    //             Keycode::D => self.x += 1,
    //             _ => {}
    //         },
    //         _ => {}
    //     }
    // }
    pub fn position_callback(&mut self, event: &EntityEvent) {
        match event {
            &EntityEvent::Position(x, y) => {
                self.x = x;
                self.y = y;
            }
            _ => {}
        }
    }
}
