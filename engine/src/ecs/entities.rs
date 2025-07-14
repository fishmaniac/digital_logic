use std::{any::Any, collections::HashMap};

use sdl3::{render::Canvas, video::{Window, WindowContext}};

use super::{components::{Components, EngineComponent, Position, Rect}, entity::Entity, events::Event};


impl ComponentStorage for Position {
    fn get_component(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities.position_components
            .get_mut(entity_id as usize).and_then(|position| position.as_mut())
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: Event) {
        todo!();
    }
}

impl ComponentStorage for Rect {
    fn get_component(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities.rect_components
            .get_mut(entity_id as usize).and_then(|position| position.as_mut())
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: Event) {
        match event {
            Event::Position(_, _) => {
                let mut position = entities.get_component_mut::<Position>(entity_id);
                match position {
                    Some(ref mut position) => position.movement_callback(&event),
                    None => (),
                }
                let position = position.cloned();
                let rect = entities.get_component_mut::<Rect>(entity_id);
                match rect {
                    Some(rect) => {
                        if let Some(position) = position {
                            rect.set_position(position);
                        }
                    }
                    None => (),
                }
            },
            Event::LeftClick(_, _) => todo!(),
        }
    }
}

pub trait ComponentStorage {
    fn get_component(entities: &mut Entities, entity_id: u32) -> Option<&mut Self>
where
        Self: Sized;
    fn listener(entities: &mut Entities, entity_id: u32, event: Event);
}

pub struct Entities {
    next_id: u32,
    pub entities: HashMap<u32, Entity>,
    components: HashMap<u32, Components>,
    pub position_components: Vec<Option<Position>>,
    pub rect_components: Vec<Option<Rect>>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: HashMap::new(),
            components: HashMap::new(),
            position_components: Vec::new(),
            rect_components: Vec::new(),
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
        self.next_id += 1;
        println!("Created entity: {}", id);
        id
    }
    pub fn create_rect_entity(&mut self, x: i32, y: i32, width: u32, height: u32) -> u32 {
        self.create_component(EngineComponent::Position(Position::new(x, y)));
        self.create_component(EngineComponent::Rect(Rect::new(x, y, width, height)));
        self.create_entity()
    }
    pub(crate) fn create_component(&mut self, component_type: EngineComponent) {
        match component_type {
            EngineComponent::Position(position) => self.position_components.push(Some(position)),
            EngineComponent::Rect(rect) => self.rect_components.push(Some(rect)),
            EngineComponent::Draggable => {},
        }
    }
    pub fn get_component_mut<Component>(&mut self, entity_id: u32) -> Option<&mut Component>
where
        Component: ComponentStorage,
    {
        Component::get_component(self, entity_id)
    }
    pub(crate) fn update(&mut self) {
        for entity in &self.entities {
        }
    }
    pub fn get_entity(&mut self, entity_id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_id)
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for rect in self.rect_components.iter() {
            if let Some(rect) = rect {
                canvas.set_draw_color(rect.color);
                canvas.fill_rect(rect.rect).expect("Failed to fill rect");
            }
        }
    }
}
