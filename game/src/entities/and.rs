use std::any::Any;

use engine::{create_entity, ecs::{
    components::ComponentStorage, entities::Entities, events::{Event, Events}, state::{StateMachine, StateStorage}
}};

use crate::{ColorRGB, Rect, EngineComponent};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AndState {
    Input,
    Process,
}

type AndStateMachine = StateMachine<AndState, StateInput, fn(&AndState) -> AndState>;

#[derive(Debug)]
struct StateInput;

pub fn transition_func(state: &AndState) -> AndState {
    match state {
        AndState::Input => AndState::Process,
        AndState::Process => AndState::Input,
    }
}

pub struct AndGate {
    id: u32,
}

impl AndGate {
    pub fn new(entities: &mut Entities, events: &mut Events) -> Self {
        let rect = Rect::new(0, 0, 75, 75, ColorRGB::new(0, 0, 255));
        let state = Box::new(AndStateMachine::new(AndState::Input, transition_func));

        let id = create_entity!(
            entities,
            (Rect, EngineComponent::Rect(rect)),
            (AndStateMachine, EngineComponent::State(state))
        );
        println!("Created And with ID {}", id);

        events.add_listener(id, Self::listener);
        events.add_listener(id, AndStateMachine::listener);

        Self {
            id,
        }
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        match event {
            Event::LeftClick(x, y) => {
                let rect = match entities.get_component_mut::<Rect>(entity_id) {
                    Some(rect) => rect,
                    None => return println!("Error, no Rect on And listener"),
                };
                if rect.contains(*x, *y) {
                    println!("And contains mouse.");
                }
            },
            _ => {},
        }
    }
}
