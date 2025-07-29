use std::any::Any;

use engine::{
    create_game_entity, ecs::{
        components::{draggable::Draggable, state::StateMachine, ComponentStorage, GameComponent}, entities::{self, Entities}, entity::Entity, events::{Event, Events}
    }
};

use crate::{ColorRGB, Rect, EngineComponent};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AndState {
    Input,
    Process,
    Output,
}

type AndStateMachine = StateMachine<AndState, AndGate>;

pub fn transition_func(state: &AndState, component: &mut AndGate) -> AndState {
    match state {
        AndState::Input => AndState::Process,
        AndState::Process => {
            component.out = component.in0 & component.in1;
            AndState::Output
        },
        AndState::Output => {
            println!("out: {}", component.out);
            AndState::Input
        },
    }
}

impl GameComponent for AndGate {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct AndGate {
    pub(crate) id: u32,
    in0: u8,
    in0_id: Option<u32>,
    in1: u8,
    in1_id: Option<u32>,
    out: u8,
    out_id: Option<u32>,
}

impl AndGate {
    pub fn new(entities: &mut Entities, events: &mut Events) {
        let rect = Rect::new(0, 0, 75, 75, ColorRGB::new(0, 0, 255));

        let next_id = entities.next_id();

        let state = Box::new(AndStateMachine::new(
            next_id,
            AndState::Input,
            transition_func
        ));

        let drag = Draggable::new();
        let and = Self {
            id: next_id,
            in0: 0,
            in0_id: None,
            in1: 0,
            in1_id: None,
            out: 0,
            out_id: None,
        };

        let id = create_game_entity!(
            entities,
            events,
            and,
            (Rect, EngineComponent::Rect(rect)),
            (AndStateMachine, EngineComponent::State(state)),
            (Draggable, EngineComponent::Draggable(drag))
        );

        println!("Created AndGate with ID {} - {}", id, next_id);
        assert!(next_id == id, "AndGate id == next_id)");

        events.add_listener(id, Self::listener);
        events.add_listener(id, AndStateMachine::listener);
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        match event {
            Event::ComponentUpdate => {
                let components = &mut entities.components;
                let and: &mut Self = match entities::get_game_component_mut(components, entity_id) {
                    Some(game_component) => game_component,
                    None => return println!("No component found for id: {}", entity_id),
                };

                match and.out_id {
                    Some(out_id) => {
                        // TODO: FIXME, component may not always Self...
                        let game_component: &mut Self = match entities::get_game_component_mut(components, out_id) {
                            Some(game_component) => game_component,
                            None => return println!("No component found for out id: {}", out_id),
                        };
                    },
                    None => todo!(),
                }
            },
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
