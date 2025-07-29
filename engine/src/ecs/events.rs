use std::collections::HashMap;

use sdl3::{
    mouse::MouseButton,
    keyboard::Keycode
};

use crate::input::Input;
use super::entities::{Entities};

pub enum Event {
    ComponentUpdate,
    StateUpdate,
    Position(i32, i32),
    LeftClick(i32, i32),
}

type Callback = fn(&mut Entities, u32, &Event);

pub struct Events {
    listeners: HashMap<u32, Vec<Callback>>, // Entity ID -> Event Callback
}

impl Events {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
    pub fn tick_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();
        let mut keys = self.handle_key_events(input);
        events.append(&mut keys);
        let mut mouse = self.handle_mouse_events(input);
        events.append(&mut mouse);

        events.push(Event::StateUpdate); // TODO: handle game events

        events
    }
    pub fn add_listener(&mut self, entity_id: u32, callback: Callback) {
        self.listeners.entry(entity_id)
            .or_insert_with(Vec::new)
            .push(callback);
    }
    pub fn handle_events(&mut self, entities: &mut Entities, events: Vec<Event>) {
        for event in events {
            self.handle_callback(entities, event);
        }
    }
    fn handle_callback(&self, entities: &mut Entities, event: Event) {
        let keys: Vec<u32> = entities.entities.keys().copied().collect();
        for entity_id in keys {
            match self.listeners.get(&entity_id) {
                Some(listeners) => {
                    for callback in listeners {
                        callback(entities, entity_id, &event)
                    }
                },
                None => {},
            };
        }
    }
    fn handle_key_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events = Vec::new();
        for key in input.pressed_keys.iter() {
            match key {
                Keycode::W => events.push(Event::Position(0, -1)),
                Keycode::A => events.push(Event::Position(-1, 0)),
                Keycode::S => events.push(Event::Position(0, 1)),
                Keycode::D => events.push(Event::Position(1, 0)),
                _ => {},
            }
        }
        events
    }
    fn handle_mouse_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events = Vec::new();
        for mouse in input.pressed_mouse.iter() {
            match mouse {
                MouseButton::Left => events.push(
                    Event::LeftClick(input.position_mouse.0, input.position_mouse.0)
                ),
                _ => {},
            }
        }
        events
    }
}
