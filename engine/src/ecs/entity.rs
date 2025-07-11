use std::any::Any;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use sdl3::rect::Rect;

use crate::renderer::Renderable;
use crate::update::{EventHandler, Updatable};

pub use super::rect_entity::RectEntity;
pub use super::texture_entity::TextureEntity;

pub trait Texturable: Renderable {
    fn texture_name(&self) -> &String;
    fn set_texture_name(&mut self, name: String);
}

pub trait Entity: Renderable {
    fn name(&self) -> &str;
    fn rect(&self) -> &Rect;
    fn rect_mut(&mut self) -> &mut Rect;
    fn x(&self) -> i32 {
        self.rect().x()
    }
    fn y(&self) -> i32 {
        self.rect().y()
    }
    fn set_x(&mut self, x: i32) {
        self.rect_mut().set_x(x);
    }
    fn set_y(&mut self, y: i32) {
        self.rect_mut().set_y(y);
    }
    fn as_texturable(&self) -> Option<&dyn Texturable> {
        None
    }
    fn as_updatable_mut(&mut self) -> Option<&mut dyn Updatable> {
        None
    }
    fn as_event_handler_mut(&mut self) -> Option<&mut dyn EventHandler> {
        None
    }
}

static IS_DRAGGING: AtomicBool = AtomicBool::new(false);

pub trait Draggable: Entity {
    fn is_dragging(&self) -> bool;
    fn set_dragging(&mut self, dragging: bool);
    fn drag_offset(&self) -> (i32, i32);
    fn set_drag_offset(&mut self, offset: (i32, i32));
    fn set_position(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }
    fn start_drag(&mut self, mouse_pos: (i32, i32)) {
        if IS_DRAGGING.load(Ordering::Relaxed) {
            return
        }

        let rect = self.rect();
        if rect.contains_point(mouse_pos) {
            self.set_dragging(true);
            self.set_drag_offset((
                mouse_pos.0 - self.x(),
                mouse_pos.1 - self.y()
            ));
            IS_DRAGGING.store(true, Ordering::Relaxed);
        }
    }
    fn stop_drag(&mut self) {
        self.set_dragging(false);
        IS_DRAGGING.store(false, Ordering::Relaxed);
    }
    fn check_drag(&mut self, mouse_pos: (i32, i32)) {
        if self.is_dragging() {
            let (offset_x, offset_y) = self.drag_offset();
            self.set_position(mouse_pos.0 - offset_x, mouse_pos.1 - offset_y);
        }
    }
}
