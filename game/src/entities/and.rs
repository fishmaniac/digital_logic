use std::{any::Any, ops::Deref};

use engine::{
    Canvas, Window, create_game_entity,
    ecs::{
        components::{
            ComponentStorage, GameComponent, draggable::Draggable, line::Line, state::StateMachine,
        },
        entities::{self, Entities},
        entity::Entity,
        events::{self, EntityEvent, Event, Events},
    },
};

use crate::{ColorRGB, EngineComponent, Rect, global_state::GlobalState};

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
        }
        AndState::Output => {
            // println!("out: {}", component.out);
            AndState::Input
        }
    }
}

impl GameComponent for AndGate {
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    is_connecting: bool,
}

impl AndGate {
    pub fn new(entities: &mut Entities, events: &mut Events) {
        let next_id = entities.next_id();
        let x = ((next_id * 1028) % 800) as i32;
        let y = 0;
        let rect = Rect::new(x, y, 75, 75, ColorRGB::new(0, 0, 255));

        let line_in0 = Line::new((x, y), (0, 0), ColorRGB::new(255, 0, 0), false);
        let line_in1 = Line::new((x, y), (0, 0), ColorRGB::new(255, 0, 0), false);
        let line_out = Line::new((x, y), (0, 0), ColorRGB::new(255, 0, 0), false);

        let state = Box::new(AndStateMachine::new(
            next_id,
            AndState::Input,
            transition_func,
        ));

        let drag = Draggable::new();
        let and = Self {
            id: next_id,
            in0: 4,
            in0_id: None,
            in1: 7,
            in1_id: None,
            out: 0,
            out_id: None,
            is_connecting: false,
        };

        let id = create_game_entity!(
            entities,
            events,
            and,
            (Rect, EngineComponent::Rect(rect)),
            (Line, EngineComponent::Line(line_in0)),
            (Line, EngineComponent::Line(line_in1)),
            (Line, EngineComponent::Line(line_out)),
            (AndStateMachine, EngineComponent::State(state)),
            (Draggable, EngineComponent::Draggable(drag))
        );

        println!("Created AndGate with ID {} - {}", id, next_id);
        assert!(next_id == id, "AndGate id == next_id)");

        events.add_global_listener(id, Self::global_listener);
        events.add_entity_listener(id, Self::entity_listener);
        events.add_entity_listener(id, AndStateMachine::entity_listener);
        events.add_global_listener(id, AndStateMachine::global_listener);
    }
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut events::EntityEvents,
        entity_id: u32,
        event: &Event,
    ) {
        match event {
            Event::ComponentUpdate => {
                let components = &mut entities.components;
                let and: &mut Self = match entities::get_game_component_mut(components, entity_id) {
                    Some(game_component) => game_component,
                    None => return println!("No component found for id: {}", entity_id),
                };

                let id = and.id;
                let out_id = and.out_id;
                let out = and.out;

                match and.out_id {
                    Some(out_id) => {
                        // TODO: FIXME, component may not always be Self...
                        let game_component: &mut Self =
                            match entities::get_game_component_mut(components, out_id) {
                                Some(game_component) => game_component,
                                None => {
                                    return println!("No component found for out id: {}", out_id);
                                }
                            };
                        println!("ID: {} - Out ID: {}: {}", id, out_id, out);
                    }
                    None => {}
                }
            }
            Event::RightClick(x, y) => {
                let (contains, rect_x, rect_y) = {
                    let rect = match entities.get_component_mut::<Rect>(entity_id) {
                        Some(r) => r,
                        None => return println!("No rect component"),
                    };
                    (rect.contains(*x, *y), rect.x(), rect.y())
                };

                let global = match entities
                    .global_state
                    .as_any_mut()
                    .downcast_mut::<GlobalState>()
                {
                    Some(state) => state,
                    None => return println!("No Global State in And"),
                };

                if global.entity_connecting.is_none() && contains {
                    global.entity_connecting = Some(entity_id)
                }

                if global.entity_connecting == Some(entity_id) {
                    let line = match entities.get_component_mut::<Line>(entity_id) {
                        Some(line) => line,
                        None => return println!("No line component"),
                    };
                    line.start = (rect_x, rect_y);
                    line.end = (*x, *y);
                    line.render = true;
                }
            }
            Event::RightClickRelease(x, y) => {
                let (contains, rect_x, rect_y) = {
                    let rect = match entities.get_component_mut::<Rect>(entity_id) {
                        Some(r) => r,
                        None => return println!("No rect component"),
                    };
                    (rect.contains(*x, *y), rect.x(), rect.y())
                };
                let global = match entities
                    .global_state
                    .as_any_mut()
                    .downcast_mut::<GlobalState>()
                {
                    Some(state) => state,
                    None => return println!("No Global State in And"),
                };

                // Do not allow connecting to itself
                if contains && global.entity_connecting != Some(entity_id) {
                    println!("Connect me: {}", entity_id);
                }
                global.entity_connecting = None;
            }
            _ => {}
        }
    }
    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent) {
        match event {
            EntityEvent::Position(_, _) => {}
            EntityEvent::Drag(_, _) => {}
        }
    }
}
