use std::collections::HashMap;

use sdl3::keyboard::Keycode;

use crate::input::InputEvent;

use super::{
    components::{EngineComponent, Position, Rect},
    entities::Entities
};

pub enum Event {
    Position(i32, i32),
    LeftClick(i32, i32),
}

type Callback = fn(&mut Entities, Event);

pub struct Events {
    listeners: HashMap<u32, Vec<Callback>>, // Entity ID -> Event Callback
}

impl Events {
    pub fn new() -> Self {
        let listeners = HashMap::new();
        Self {
            listeners,
        }
    }

    pub fn setup_listeners(&mut self) {
    }

    pub fn add_listener(&mut self, id: u32, callback: Callback) {
        self.listeners.entry(id)
            .or_insert_with(Vec::new)
            .push(callback);
    }

    // TODO: ensure new component is only used to update and not replace
    // could just use engine component
    // pub(crate) fn event_components(event_type: Event) -> Vec<EngineComponent> {
    //     let mut events = Vec::new();
    //     match event_type {
    //         Event::Position(x, y) => {
    //             events.push(EngineComponent::Position(Position::new(x, y)));
    //             events.push(EngineComponent::Rect(Rect::new(x, y, 0, 0)));
    //         }
    //     }
    //     events
    // }
    pub fn handle_events(&mut self, entities: &mut Entities, events: Vec<Event>) {
        for event in events {
            self.handle_callback(entities, event);
        }
    }
    pub fn handle_callback(&self, entities: &mut Entities, event: Event) {
        let keys: Vec<u32> = entities.entities.keys().copied().collect();
        for entity_id in keys {
            match event {
                Event::Position(_, _) => {
                    let mut position = entities.get_component_mut::<Position>(entity_id);
                    match position {
                        Some(ref mut position) => position.movement_callback(&event),
                        None => (),
                    }
                    let position = position.cloned();
                    let rect = entities.get_component_mut::<Rect>(entity_id);
                    match rect {
                        Some(rect) => {
                            if let Some(position) = position {
                                rect.set_position(position);
                            }
                        }
                        None => (),
                    }
                },
                Event::LeftClick(x, y) => {
                    let rect = entities.get_component_mut::<Rect>(entity_id);
                    match rect {
                        Some(rect) => {
                            rect.mouse_contains(x, y);
                        }
                        None => (),
                    }
                },
            }
        }
    }
    pub fn handle_input_events(&mut self, input_events: Vec<InputEvent>) -> Vec<Event> {
        let mut events = Vec::new();
        for input in input_events {
            match input {
                InputEvent::KeyDown(keycode) => {
                    match keycode {
                        Keycode::W => events.push(Event::Position(0, -1)),
                        Keycode::A => events.push(Event::Position(-1, 0)),
                        Keycode::S => events.push(Event::Position(0, 1)),
                        Keycode::D => events.push(Event::Position(1, 0)),
                        _ => {},
                    }
                },
                InputEvent::KeyUp(keycode) => {},
                InputEvent::MouseMotion { x, y } => {},
                InputEvent::MouseButtonDown { x, y, button } => {
                    match button {
                        sdl3::mouse::MouseButton::Left => events.push(Event::LeftClick(x, y)),
                        _ => {},
                    };
                },
                InputEvent::MouseButtonUp { x, y, button } => {},
            }
        }
        events
    }

    // pub fn trigger_event(
    //     &mut self,
    //     // entities: Entities,
    //     entity: &mut Entity,
    //     event_type: Event
    // ) {
    //     match self.listeners.get(&entity.id) {
    //         Some(callbacks) => {
    //             let mut component_events = Self::event_components(event_type);
    //             for callback in callbacks {
    //                 for component in &mut component_events {
    //                     callback(component);
    //                 }
    //             }
    //         },
    //         None => println!("No callback for entity: {}", entity.id),
    //     }
    // }
    // pub fn update(&mut self, entities: &mut Entities, event_type: Vec<Event>) {
    //     let mut component_events = Self::event_components(event_type);
    // }
}
