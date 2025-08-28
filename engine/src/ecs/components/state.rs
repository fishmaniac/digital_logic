use std::{any::Any, fmt::Debug, hash::Hash};

use crate::ecs::{
    components::{ComponentStorage, EngineComponent, GameComponent},
    entities::{self, Entities},
    events::{self, EntityEvent, Event, Events},
};

// pub trait StateHandler {
// }

type TransitionFn<State, Entity> = fn(&State, &mut Entity) -> State;

#[derive(Debug)]
pub struct StateMachine<State, Entity>
where
    State: Eq + Hash,
    Entity: GameComponent,
{
    entity_id: u32,
    pub state: State,
    transition_callback: TransitionFn<State, Entity>,
}

impl<State, Entity> StateMachine<State, Entity>
where
    State: Eq + Hash + Debug,
    Entity: GameComponent,
{
    pub fn new(
        entity_id: u32,
        state: State,
        transition_callback: TransitionFn<State, Entity>,
    ) -> Self {
        StateMachine {
            entity_id,
            state,
            transition_callback,
        }
    }
    pub fn execute(&mut self, component: &mut Entity) {
        self.state = (self.transition_callback)(&self.state, component);
        // println!("New state: {:?}", self.state);
    }
}

impl<State, Entity> ComponentStorage for StateMachine<State, Entity>
where
    State: Eq + Hash + 'static + Debug,
    Entity: GameComponent + 'static + Debug,
{
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        Self::get_state_machine(&mut entities.state_components, entity_id)
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<Self>())
    }
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut events::EntityEvents,
        entity_id: u32,
        event: &Event,
    ) {
        match event {
            Event::StateUpdate => {
                let components = &mut entities.components;
                let state_components = &mut entities.state_components;

                let game_component: &mut Entity =
                    match entities::get_game_component_mut(components, entity_id) {
                        Some(game_component) => game_component,
                        None => {
                            return println!(
                                "No state machine component found for id: {}",
                                entity_id
                            );
                        }
                    };

                let state = match Self::get_state_machine(state_components, entity_id)
                    .and_then(|boxed| boxed.as_any_mut().downcast_mut::<Self>())
                {
                    Some(state) => state.execute(game_component), // State transition
                    None => println!("No state machine found for id: {}", entity_id),
                };
            }
            _ => {}
        }
    }

    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent) {}

    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
        entities.create_component(component);
    }
}

pub trait StateStorage: Any + Debug {
    fn get_state_machine(
        components: &mut Vec<Option<Box<dyn StateStorage>>>,
        entity_id: u32,
    ) -> Option<&mut Box<dyn StateStorage>>
    where
        Self: Sized;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<State, Entity> StateStorage for StateMachine<State, Entity>
where
    State: Eq + Hash + 'static + Debug,
    Entity: GameComponent + 'static + Debug,
{
    fn get_state_machine(
        components: &mut Vec<Option<Box<dyn StateStorage>>>,
        entity_id: u32,
    ) -> Option<&mut Box<dyn StateStorage>>
    where
        Self: Sized,
    {
        components
            .get_mut(entity_id as usize)
            .and_then(|state| state.as_mut())
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
