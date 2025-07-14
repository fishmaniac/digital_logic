use std::marker::PhantomData;

pub struct StateMachine<CurrentState, Entity> {
    pub _state: PhantomData<CurrentState>,
    pub _entity: PhantomData<Entity>,
}

pub trait State {
    fn transition<NextState, Entity>(&self) -> StateMachine<NextState, Entity>
    where
        NextState: 'static,
        Entity: 'static;
}


impl<CurrentState, Entity> StateMachine<CurrentState, Entity> {
    pub fn new() -> Self {
        StateMachine {
            _state: PhantomData,
            _entity: PhantomData,
        }
    }

    pub fn transition<NextState>(&self) -> StateMachine<NextState, Entity> {
        println!("Transitioning to the next state.");
        StateMachine {
            _state: PhantomData,
            _entity: PhantomData,
        }
    }
}
