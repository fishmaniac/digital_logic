use std::collections::HashMap;

use sdl3::{keyboard::Keycode, mouse::MouseButton};

use super::entities::Entities;
use crate::input::Input;

pub enum Event {
    ComponentUpdate,
    StateUpdate,
    KeyCode(Keycode),
    KeyCodeRelease(Keycode),
    LeftClick(i32, i32),
    RightClick(i32, i32),
    LeftClickRelease(i32, i32),
    RightClickRelease(i32, i32),
}

pub enum EntityEvent {
    Position(i32, i32),
    Drag(i32, i32),
}

type GlobalCallback = fn(&mut Entities, &mut EntityEvents, u32, &Event);
type EntityCallback = fn(&mut Entities, u32, &EntityEvent);
type GlobalListeners = HashMap<u32, Vec<GlobalCallback>>;
pub type EntityListeners = HashMap<u32, Vec<EntityCallback>>;
type GlobalEvents = Vec<Event>;
pub type EntityEvents = HashMap<u32, Vec<EntityEvent>>;

pub struct Events {
    global_listeners: GlobalListeners, // Entity ID -> Event Callback
    entity_listeners: EntityListeners, // Entity ID -> Event Callback
    entity_events: EntityEvents,
    global_events: GlobalEvents,
}

impl Events {
    pub fn new() -> Self {
        Self {
            global_listeners: HashMap::new(),
            entity_listeners: HashMap::new(),
            entity_events: HashMap::new(),
            global_events: Vec::new(),
        }
    }
    pub fn input_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();
        let mut keys = self.handle_key_events(input);
        events.append(&mut keys);
        let mut mouse = self.handle_mouse_events(input);
        events.append(&mut mouse);

        events
    }
    fn generate_events(&mut self, input: &Input) {
        // Reset global events
        self.global_events = self.input_events(input);
        self.global_events.push(Event::StateUpdate);
        self.global_events.push(Event::ComponentUpdate);
    }
    pub fn add_global_listener(&mut self, entity_id: u32, callback: GlobalCallback) {
        self.global_listeners
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(callback);
    }
    pub fn add_entity_listener(&mut self, entity_id: u32, callback: EntityCallback) {
        self.entity_listeners
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(callback);
    }
    pub fn create_entity_event(
        entity_events: &mut EntityEvents,
        entity_id: u32,
        event: EntityEvent,
    ) {
        entity_events
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(event);
    }
    pub fn handle_events(&mut self, entities: &mut Entities, input: &Input) {
        // println!(
        //     "Handling events: Global {} - Entity {}",
        //     self.global_events.len(),
        //     self.entity_events.len()
        // );
        self.generate_events(input);
        // TODO: move listeners to global or entity
        // TODO: refactor to one handle_callback
        // TODO: handle new events
        for event in &mut self.global_events {
            Self::handle_global_callback(
                &mut self.global_listeners,
                &mut self.entity_events,
                entities,
                event,
            );
        }
        for (id, events) in self.entity_events.drain() {
            for event in events {
                if let Some(listeners) = self.entity_listeners.get(&id) {
                    for callback in listeners {
                        callback(entities, id, &event);
                    }
                }
            }
        }
    }
    fn handle_global_callback(
        listeners: &mut GlobalListeners,
        entity_events: &mut EntityEvents,
        entities: &mut Entities,
        event: &Event,
    ) {
        let keys: Vec<u32> = entities.entities.keys().copied().collect();
        for entity_id in keys {
            if let Some(listeners) = listeners.get(&entity_id) {
                for callback in listeners {
                    callback(entities, entity_events, entity_id, event);
                }
            };
        }
    }
    fn handle_key_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events = Vec::new();
        for key in input.pressed_keys.iter() {
            events.push(Event::KeyCode(*key));
        }
        events
    }
    fn handle_mouse_events(&mut self, input: &Input) -> Vec<Event> {
        let mut events = Vec::new();
        for mouse in input.pressed_mouse.iter() {
            match mouse {
                MouseButton::Left => events.push(Event::LeftClick(
                    input.position_mouse.0,
                    input.position_mouse.1,
                )),
                MouseButton::Right => events.push(Event::RightClick(
                    input.position_mouse.0,
                    input.position_mouse.1,
                )),
                _ => {}
            }
        }

        for mouse in &input.just_released_mouse {
            match mouse {
                MouseButton::Left => events.push(Event::LeftClickRelease(
                    input.position_mouse.0,
                    input.position_mouse.1,
                )),
                MouseButton::Right => events.push(Event::RightClickRelease(
                    input.position_mouse.0,
                    input.position_mouse.1,
                )),
                _ => {}
            }
        }
        events
    }
}
