use std::marker::PhantomData;

use engine::ecs::{
    components::{ColorRGB, EngineComponent, Rect},
    entities::Entities,
    events::{Event, Events}, state::{State, StateMachine}
};

struct StateInput;
struct StateProcess;
struct StateOutput;

impl State for AndGate {
    fn transition<NextState, Entity>(&self) -> StateMachine<NextState, Entity>
        where
            NextState: 'static,
            Entity: 'static {
        StateMachine::new()
    }
}

pub struct AndGate {
    id: u32,
    state: StateMachine<StateInput, AndGate>,
}

impl AndGate {
    pub fn new(entities: &mut Entities, events: &mut Events) -> Self {
        let id = entities.create::<Rect>(EngineComponent::Rect(
            Rect::new(0, 0, 75, 75, ColorRGB::new(0, 0, 255))
        ));
        events.add_listener(id, Self::listener);

        Self {
            id,
            state: StateMachine::new(),
        }
    }
    fn update(&mut self) {
        self.state = self.state.transition();
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        match event {
            Event::Position(_, _) => {},
            Event::LeftClick(x, y) => {
                let rect = match entities.get_component_mut::<Rect>(entity_id) {
                    Some(rect) => rect,
                    None => return println!("Error, no Rect on And listener"),
                };
                if rect.mouse_contains(*x, *y) {
                    println!("And contains mouse.");
                }
            },
        }
    }
}
