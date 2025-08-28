use crate::ecs::{
    components::{
        ComponentStorage, EngineComponent, line::Line, position::Position, rect::Rect,
        state::StateMachine,
    },
    entities::Entities,
    events::{EntityEvent, Event, Events},
};

#[derive(Debug)]
pub struct Draggable {
    is_dragging: bool,
    x_offset: i32,
    y_offset: i32,
}

impl Draggable {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            x_offset: 0,
            y_offset: 0,
        }
    }
}

// pub enum DragState {
//     Placed,
//     Dragging,
// }
//
// type DragStateMachine = StateMachine<DragState, Draggable>;
//
// pub fn transition_func(state: &DragState, component: &mut Draggable) -> DragState {
//     match state {
//         DragState::Placed => DragState::Placed,
//         DragState::Dragging => DragState::Dragging,
//     }
// }

impl ComponentStorage for Draggable {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities
            .drag_components
            .get_mut(entity_id as usize)
            .and_then(|position| position.as_mut())
    }
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut crate::ecs::events::EntityEvents,
        entity_id: u32,
        event: &Event,
    ) {
        match event {
            Event::LeftClick(x, y) => {
                let (contains, rect_x, rect_y) = {
                    let rect = match entities.get_component_mut::<Rect>(entity_id) {
                        Some(r) => r,
                        None => return println!("No rect component"),
                    };
                    (rect.contains(*x, *y), rect.x(), rect.y())
                };

                let drag = match entities.get_component_mut::<Draggable>(entity_id) {
                    Some(d) => d,
                    None => return println!("No drag component"),
                };

                if contains && !drag.is_dragging {
                    drag.x_offset = rect_x - *x;
                    drag.y_offset = rect_y - *y;
                    drag.is_dragging = true;
                    println!(
                        "Started dragging with offset {}, {}",
                        drag.x_offset, drag.y_offset
                    );
                }

                if drag.is_dragging {
                    let new_x = drag.x_offset + *x;
                    let new_y = drag.y_offset + *y;
                    Events::create_entity_event(
                        entity_events,
                        entity_id,
                        EntityEvent::Position(new_x, new_y),
                    );
                }
            }
            Event::LeftClickRelease(x, y) => {
                let (contains, rect_x, rect_y) = {
                    let rect = match entities.get_component_mut::<Rect>(entity_id) {
                        Some(r) => r,
                        None => return println!("No rect component"),
                    };
                    (rect.contains(*x, *y), rect.x(), rect.y())
                };

                let drag = match entities.get_component_mut::<Draggable>(entity_id) {
                    Some(d) => d,
                    None => return println!("No drag component"),
                };

                if drag.is_dragging {
                    drag.x_offset = rect_x;
                    drag.y_offset = rect_y;
                    drag.is_dragging = false;
                }
            }
            _ => {}
        }
    }
    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent) {}
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
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
