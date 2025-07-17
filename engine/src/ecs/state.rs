use std::{any::Any, fmt::Debug, hash::Hash};

use ecs::events;

use crate::ecs::components::ComponentStorage;
use super::{components::EngineComponent, entities::Entities, events::Event};
use crate::ecs;

#[derive(Debug)]
pub struct StateMachine<State, Event, Function> 
where 
    Function: Fn(&State) -> State,
    State: Eq + Hash,
    {
    state: State,
    transition_callback: Function,
    events: Vec<Event>,
}


impl<State, Event, Function> StateMachine<State, Event, Function>
where
    State: Eq + Hash + Debug,
    Event: Debug,
    Function: Fn(&State) -> State,
{
    pub fn new(state: State, function: Function) -> Self {
        StateMachine {
            state,
            transition_callback: function,
            events: Vec::new(),
        }
    }
    pub fn transition(&mut self) -> &mut Self {
        self.state = (self.transition_callback)(&self.state);
        self
    }
    pub fn add_event(&mut self, event: Event) -> &mut Self {
        self.events.push(event);
        self
    }
    pub fn execute(&mut self) {
        self.transition();
        println!("New state: {:?}", self.state);
    }
}

impl<State, Event: 'static, Function> ComponentStorage for StateMachine<State, Event, Function>
where
    State: Eq + Hash + 'static + Debug,
    Event: Debug,
    Function: Fn(&State) -> State + 'static + Debug,
{
    fn get_mut(
        entities: &mut Entities,
        entity_id: u32,
    ) -> Option<&mut StateMachine<State, Event, Function>> {
        StateMachine::<State, Event, Function>::get_state_machine(entities, entity_id)
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<Self>())
    }

    fn listener(entities: &mut Entities, entity_id: u32, event: &events::Event) {
        match event {
            events::Event::StateUpdate => {
                match entities
                    .state_components
                    .get_mut(entity_id as usize)
                    .and_then(|state| state.as_mut()) {
                    Some(state) => {
                        state.execute();
                    },
                    None => println!("No state machine found for id: {}.", entity_id),
                }
            },
            _ => {},
        }
    }

    fn create(entities: &mut Entities, component: EngineComponent) {
        entities.create_component(component);
        // entities.create_entity()
    }
}

pub trait StateStorage: Any + Debug {
    fn get_state_machine(
        entities: &mut Entities,
        entity_id: u32
    ) -> Option<&mut Box<dyn StateStorage>>
where
        Self: Sized;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn execute(&mut self);
}

impl <State, Event: 'static, Function>StateStorage for StateMachine<State, Event, Function>
where 
    State: Eq + Hash + 'static + Debug,
    Event: Debug,
    Function: Fn(&State) -> State + 'static + Debug,
{
    fn get_state_machine(
        entities: &mut Entities,
        entity_id: u32
    ) -> Option<&mut Box<dyn StateStorage>>
where
        Self: Sized {
        entities.state_components
            .get_mut(entity_id as usize).and_then(|state| state.as_mut())
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn execute(&mut self) {
        self.execute();
    }
}
