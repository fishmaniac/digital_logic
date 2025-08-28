use std::{any::Any, collections::HashMap};

use sdl3::{render::Canvas, video::Window};

use super::{
    components::{EngineComponent, rect::Rect},
    entity::Entity,
};
use crate::{
    ecs::{
        components::{
            ComponentStorage, GameComponent, draggable::Draggable, line::Line, position::Position,
            state::StateStorage, texture_rect::TextureRect,
        },
        events::Events,
    },
    renderer::Renderable,
};

#[macro_export]
macro_rules! create_entity {
    ($entities:expr, $events:expr, $( ($type:ty, $component:expr) ),+ $(,)?) => {{
        $(
            $entities.create_engine_component::<$type>($events, $component);
        )+
        $entities.create_entity()
    }};
}

#[macro_export]
macro_rules! create_game_entity {
    ($entities:expr, $events:expr, $game_component:expr, $( ($type:ty, $component:expr) ),+ $(,)?) => {{
        let id = {
            $(
                $entities.create_engine_component::<$type>($events, $component);
            )+
            $entities.create_entity()
        };
        $entities.create_game_component(id, Box::new($game_component));
        id
    }};
}

pub trait GlobalState: Any {}

impl dyn GlobalState {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Entities {
    next_id: u32,
    pub entities: HashMap<u32, Entity>,
    pub components: HashMap<u32, Box<dyn GameComponent>>,
    pub position_components: Vec<Option<Position>>,
    pub rect_components: Vec<Option<Rect>>,
    pub line_components: Vec<Option<Line>>,
    pub texture_components: Vec<Option<TextureRect>>,
    pub state_components: Vec<Option<Box<dyn StateStorage>>>,
    pub drag_components: Vec<Option<Draggable>>,
    pub global_state: Box<dyn GlobalState + 'static>,
}

impl Entities {
    pub fn new(global_state: impl GlobalState + 'static) -> Self {
        Self {
            next_id: 0,
            entities: HashMap::new(),
            components: HashMap::new(),
            position_components: Vec::new(),
            rect_components: Vec::new(),
            line_components: Vec::new(),
            texture_components: Vec::new(),
            state_components: Vec::new(),
            drag_components: Vec::new(),
            global_state: Box::new(global_state),
        }
    }
    fn fill_component_capacity(&mut self, id: u32) {
        if id as usize + 1 > self.rect_components.len() {
            self.rect_components.push(None);
        }
        if id as usize + 1 > self.line_components.len() {
            self.line_components.push(None);
        }
        if id as usize + 1 > self.texture_components.len() {
            self.texture_components.push(None);
        }
        if id as usize + 1 > self.position_components.len() {
            self.position_components.push(None);
        }
        if id as usize + 1 > self.state_components.len() {
            self.state_components.push(None);
        }
        if id as usize + 1 > self.drag_components.len() {
            self.drag_components.push(None);
        }
    }
    pub fn next_id(&self) -> u32 {
        self.next_id
    }
    pub fn create_entity(&mut self) -> u32 {
        let entity = Entity::new(self.next_id);
        let id = entity.id;
        self.entities.insert(entity.id, entity);
        self.next_id += 1;

        self.fill_component_capacity(id);

        println!("rect components: {:?}", self.rect_components.len() - 1);
        println!(
            "position components: {:?}",
            self.position_components.len() - 1
        );
        println!("state components: {:?}", self.state_components.len() - 1);
        println!("drag components: {:?}", self.drag_components.len() - 1);
        println!("created entity: {}", id);

        id
    }
    pub fn get_entity(&mut self, entity_id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_id)
    }
    pub fn create_engine_component<Component: ComponentStorage>(
        &mut self,
        events: &mut Events,
        component: EngineComponent,
    ) {
        Component::create(self, component, events);
        // TODO: FIXME reliance on next_id
        println!("Adding listener: {}", self.next_id);
        events.add_global_listener(self.next_id, Component::global_listener);
        events.add_entity_listener(self.next_id, Component::entity_listener);
    }
    pub(crate) fn create_component(&mut self, component_type: EngineComponent) {
        println!("Adding component: {:?}", component_type);
        match component_type {
            EngineComponent::State(state) => self.state_components.push(Some(state)),
            EngineComponent::Position(position) => self.position_components.push(Some(position)),
            EngineComponent::Rect(rect) => self.rect_components.push(Some(rect)),
            EngineComponent::Line(line) => self.line_components.push(Some(line)),
            EngineComponent::Texture(texture) => self.texture_components.push(Some(texture)),
            EngineComponent::Draggable(drag) => self.drag_components.push(Some(drag)),
        }
    }
    pub fn get_component_mut<Component>(&mut self, entity_id: u32) -> Option<&mut Component>
    where
        Component: ComponentStorage,
    {
        Component::get_mut(self, entity_id)
    }
    pub fn get_game_component_mut(
        &mut self,
        entity_id: u32,
    ) -> Option<&mut Box<dyn GameComponent>> {
        return self.components.get_mut(&entity_id);
    }
    pub fn create_game_component(&mut self, entity_id: u32, component: Box<dyn GameComponent>) {
        match self.components.insert(entity_id, component) {
            Some(_) => println!("Entity {} already has GameComponent", entity_id),
            None => {}
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for rect in self.rect_components.iter() {
            if let Some(rect) = rect {
                rect.render(canvas);
            }
        }
        for line in self.line_components.iter() {
            if let Some(line) = line {
                line.render(canvas);
            }
        }
    }
}

pub fn get_game_component_mut<ComponentType>(
    components: &mut HashMap<u32, Box<dyn GameComponent>>,
    entity_id: u32,
) -> Option<&mut ComponentType>
where
    ComponentType: GameComponent,
{
    components
        .get_mut(&entity_id)
        .and_then(|component| component.as_any_mut().downcast_mut::<ComponentType>())
}
