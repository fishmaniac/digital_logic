// pub struct UpdateManager {
//
// }

use sdl3::mouse::MouseButton;

use crate::{ecs::entity::Draggable, input::GameEvent, ecs::entity_manager::EntityManager};

pub trait Updatable {
    fn update(&mut self);
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &GameEvent, entities: &mut EntityManager);
}

pub trait DraggableEventHandler {
    fn handle_drag_event(&mut self, event: &GameEvent);
}

// TODO: type states

impl<T: Draggable> DraggableEventHandler for T {
    fn handle_drag_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::MouseMotion { x, y } => {
                let mouse_pos = (*x as i32, *y as i32);
                self.check_drag(mouse_pos);
            },
            GameEvent::MouseButtonDown { x, y, button: MouseButton::Left } => {
                let mouse_pos = (*x, *y);
                self.start_drag(mouse_pos);
            }
            GameEvent::MouseButtonUp { button: MouseButton::Left, .. } => {
                self.stop_drag();
            }
            _ => {}
        }
    }
}

