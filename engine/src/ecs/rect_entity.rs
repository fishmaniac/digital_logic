use std::{any::Any, collections::HashMap};

use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window
};

use crate::renderer::Renderable;

use super::entity::Entity;

#[derive(Clone)]
pub struct RectEntity {
    pub name: String,
    pub z: u32,
    pub rect: Rect,
    pub color: Color,
}

impl RectEntity {
    pub fn new(name: String, x: i32, y: i32, z: u32, width: u32, height: u32, color: Color) -> Self {
        RectEntity {
            name,
            z,
            rect: Rect::new(x, y, width, height),
            color,
        }
    }

}

impl Renderable for RectEntity {
    fn render(&self, canvas: &mut Canvas<Window>, _textures: &HashMap<String, Texture>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).expect("Failed to fill rect");
    }
    fn z(&self) -> u32 {
        self.z
    }
}

impl Entity for RectEntity {
    fn rect(&self) -> &Rect {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn x(&self) -> i32 {
        self.rect.x
    }
    fn y(&self) -> i32 {
        self.rect.y
    }
    fn set_x(&mut self, x: i32) {
        self.rect.x = x;
    }
    fn set_y(&mut self, y: i32) {
        self.rect.y = y;
    }
}
