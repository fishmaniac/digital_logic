use engine::ecs::entities;

pub struct GlobalState {
    pub entity_dragging: Option<u32>,
    pub entity_connecting: Option<u32>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            entity_dragging: None,
            entity_connecting: None,
        }
    }
}

impl entities::GlobalState for GlobalState {}
