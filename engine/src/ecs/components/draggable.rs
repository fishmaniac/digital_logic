use crate::ecs::{components::{position::Position, rect::Rect, ComponentStorage, EngineComponent}, entities::Entities, events::Event};

#[derive(Debug)]
pub struct Draggable {
    is_dragging: bool,
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
        }
    }
}

impl ComponentStorage for Draggable {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities.drag_components
            .get_mut(entity_id as usize).and_then(|position| position.as_mut())
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        match event {
            Event::LeftClick(x, y) => {
                let rect = entities.get_component_mut::<Rect>(entity_id);
                match rect {
                    Some(rect) => {
                        if rect.contains(*x, *y) {
                            println!("Drag rect contains mouse");
                        }
                    }
                    None => (),
                }
                println!("Drag update: {} {}", x, y);
            },
            _ => {},
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent) {
        // entities.add_listener(entity_id, Self::listener);
        // match &component {
        //     EngineComponent::Rect(rect) => {
        //         entities.create_component(EngineComponent::Position(Position::new(
        //             rect.rect.x,
        //             rect.rect.y
        //         )));
        //     },
        //     _ => println!("Invalid component for Rect"),
        // };
        entities.create_component(component);
    }
}
