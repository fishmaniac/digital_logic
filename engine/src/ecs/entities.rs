use std::{any::Any, collections::HashMap};

use sdl3::{render::Canvas, video::Window};

use super::{components::{rect::Rect, Components, EngineComponent}, entity::Entity, state::{StateMachine, StateStorage}};
use crate::{ecs::components::{position::Position, ComponentStorage}, renderer::Renderable};

#[macro_export]
macro_rules! create_entity {
    ($entities:expr, $( ($type:ty, $component:expr) ),+ $(,)?) => {{
        $(
            $entities.create_engine_component::<$type>($component);
        )+
        $entities.create_entity()
    }};
}

pub struct Entities {
    next_id: u32,
    pub entities: HashMap<u32, Entity>,
    components: HashMap<u32, Components>,
    pub position_components: Vec<Option<Position>>,
    pub rect_components: Vec<Option<Rect>>,
    pub state_components: Vec<Option<Box<dyn StateStorage>>>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: HashMap::new(),
            components: HashMap::new(),
            position_components: Vec::new(),
            rect_components: Vec::new(),
            state_components: Vec::new(),
        }
    }
    pub fn create_entity(&mut self) -> u32 {
        let entity = Entity::new(self.next_id);
        let id = entity.id;
        self.entities.insert(entity.id, entity);

        if id as usize + 1 > self.rect_components.len() {
            self.rect_components.push(None);
        }
        if id as usize + 1 > self.position_components.len() {
            self.position_components.push(None);
        }
        if id as usize + 1 > self.state_components.len() {
            self.state_components.push(None);
        }
        self.next_id += 1;

        println!("rect components: {:?}", self.rect_components.len() - 1);
        println!("position components: {:?}", self.position_components.len() - 1);
        println!("state components: {:?}", self.state_components.len() - 1);

        println!("Created entity: {}", id);
        id
    }
    pub fn create_engine_component<Component: ComponentStorage>(
        &mut self,
        component: EngineComponent
    ) {
        Component::create(self, component)
    }
    pub(crate) fn create_component(&mut self, component_type: EngineComponent) {
        println!("Adding component: {:?}", component_type);
        match component_type {
            EngineComponent::State(state) => self.state_components.push(Some(state)),
            EngineComponent::Position(position) => self.position_components.push(Some(position)),
            EngineComponent::Rect(rect) => self.rect_components.push(Some(rect)),
            EngineComponent::Draggable => {},
        }
    }
    pub fn get_component_mut<Component>(&mut self, entity_id: u32) -> Option<&mut Component>
    where
        Component: ComponentStorage,
    {
        Component::get_mut(self, entity_id)
    }
    pub fn get_entity(&mut self, entity_id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_id)
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for rect in self.rect_components.iter() {
            if let Some(rect) = rect {
                rect.render(canvas);
            }
        }
    }
}

