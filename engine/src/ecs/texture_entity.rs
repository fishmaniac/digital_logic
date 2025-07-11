use std::{any::Any, collections::HashMap};
use sdl3::{rect::Rect, render::{Canvas, FRect, Texture}, video::Window};

use crate::renderer::Renderable;
use super::entity::{Entity, RectEntity, Texturable};

#[derive(Clone)]
pub struct TextureEntity {
    pub rect_entity: RectEntity,
    pub texture_name: String,
}

impl TextureEntity {
    pub fn new(
        entity: RectEntity,
        texture_name: String,
    ) -> Self {
        TextureEntity {
            rect_entity: entity,
            texture_name,
        }
    }
}

impl Texturable for TextureEntity {
    fn texture_name(&self) -> &String {
        &self.texture_name
    }
    fn set_texture_name(&mut self, name: String) {
        self.texture_name = name
    }
}

impl Entity for TextureEntity {
    fn rect(&self) -> &Rect {
        &self.rect_entity.rect
    }
    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect_entity.rect
    }
    fn name(&self) -> &str {
        self.rect_entity.name()
    }
    fn as_texturable(&self) -> Option<&dyn Texturable> {
        Some(self)
    }
}

impl Renderable for TextureEntity {
    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        textures: &HashMap<String, Texture>,
    ) {
        canvas.set_draw_color(self.rect_entity.color);
        canvas.fill_rect(self.rect_entity.rect).expect("Failed to fill rect");

        let texture = match textures.get(&self.texture_name) {
            Some(texture) => texture,
            None => {
                println!("Texture not found: {}.", self.texture_name);
                return
            }
        };
        canvas.copy(
            texture,
            None,
            Some(FRect::from(self.rect_entity.rect))
        ).expect("Failed to copy texture");
    }
    fn z(&self) -> u32 {
        self.rect_entity.z
    }
}
