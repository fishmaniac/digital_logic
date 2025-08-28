use sdl3::{pixels::Color, rect::Rect as SdlRect, render::Canvas, video::Window};

use super::ColorRGB;
use super::{
    super::components::{ComponentStorage, EngineComponent},
    position::Position,
};
use crate::ecs::components::rect::Rect;
use crate::ecs::events::{EntityEvent, Events};
use crate::{
    ecs::{entities::Entities, events::Event},
    renderer::Renderable,
};

#[derive(Debug)]
pub struct TextureRect {
    pub texture: String,
}
impl TextureRect {
    pub fn new(texture: String, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { texture }
    }
}

impl Renderable for TextureRect {
    fn render(&self, canvas: &mut Canvas<Window>) {
        // let rect = get_engin
        // canvas.set_draw_color(self.color);
        // let _ = canvas.fill_rect(self.rect);
    }
}

impl ComponentStorage for TextureRect {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities
            .texture_components
            .get_mut(entity_id as usize)
            .and_then(|texture| texture.as_mut())
    }
    fn global_listener(
        entities: &mut Entities,
        entity_events: &mut crate::ecs::events::EntityEvents,
        entity_id: u32,
        event: &Event,
    ) {
    }
    fn entity_listener(entities: &mut Entities, entity_id: u32, event: &EntityEvent) {
        match event {
            _ => {}
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
        // match &component {
        //     EngineComponent::Texture(texture) => {
        //         let position = EngineComponent::Rect(Rect::new());
        //         entities.create_engine_component::<Rect>(events, position);
        //     }
        //     _ => println!("Invalid component for TextureRect"),
        // };
        entities.create_component(component);
    }
}
