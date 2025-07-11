use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{input::GameEvent, renderer::{Renderable, Renderer}, update::EventHandler};

use super::{entity::Entity, texture_entity::TextureEntity};

pub struct EntityManager {
    pub entities: HashMap<String, Rc<RefCell<dyn Entity>>>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::new(),
        }
    }

    pub fn create_entity<T>(&mut self, renderer: &mut Renderer, entity: T) -> Rc<RefCell<dyn Entity>> where T: Entity + 'static, {
        let name = entity.name().to_string();

        let entity_rc: Rc<RefCell<dyn Entity>> = Rc::new(RefCell::new(entity));
        println!("Adding entity: {}", name);
        self.entities.insert(name, Rc::clone(&entity_rc));

        let renderable: Rc<RefCell<dyn Renderable>> = entity_rc.clone();
        renderer.add_renderable(renderable);

        if let Some(entity_rc) = entity_rc.borrow().as_texturable() {
            println!("Adding texture: {}", entity_rc.texture_name().to_string());
            renderer.add_texture(entity_rc.texture_name().to_string());
        }

        entity_rc
    }

    pub fn get_entity(&self, name: &str) -> Option<Rc<RefCell<dyn Entity>>> {
        self.entities.get(name).map(Rc::clone)
    }

    pub fn update_entities(&mut self) {
        for entity_rc in self.entities.values() {
            let mut entity = entity_rc.borrow_mut();

            if let Some(entity) = entity.as_updatable_mut() {
                entity.update();
            }
        }
    }

    pub fn handle_events(&mut self, events: Vec<GameEvent>) {
        for event in events {
            let entity_refs: Vec<_> = self.entities.values().cloned().collect();
            for entity_rc in entity_refs {
                let mut entity = entity_rc.borrow_mut();

                if let Some(handler) = entity.as_event_handler_mut() {
                    handler.handle_event(&event, self);
                }
            }
        }
    }
}
