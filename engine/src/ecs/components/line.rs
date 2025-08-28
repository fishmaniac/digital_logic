use sdl3::{pixels::Color, rect::Rect as SdlRect, render::Canvas, video::Window};

use super::ColorRGB;
use super::{
    super::components::{ComponentStorage, EngineComponent, GameComponent},
    position::Position,
};
use crate::ecs::entities;
use crate::ecs::events::{EntityEvent, Events};
use crate::{
    ecs::{entities::Entities, events::Event},
    renderer::Renderable,
};

#[derive(Debug)]
pub struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub color: Color,
    pub render: bool,
}
impl Line {
    pub fn new(start: (i32, i32), end: (i32, i32), color: ColorRGB, render: bool) -> Self {
        Self {
            start,
            end,
            color: color.into(),
            render,
        }
    }
}

impl Renderable for Line {
    fn render(&self, canvas: &mut Canvas<Window>) {
        if self.render {
            canvas.set_draw_color(self.color);
            let _ = canvas.draw_line(self.start, self.end);
        }
    }
}

impl ComponentStorage for Line {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities
            .line_components
            .get_mut(entity_id as usize)
            .and_then(|position| position.as_mut())
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
            EntityEvent::Position(x, y) => {
                let line = match entities.get_component_mut::<Line>(entity_id) {
                    Some(line) => line.start = (*x, *y),
                    None => return println!("Invalid component for Line"),
                };
            }
            EntityEvent::Drag(x, y) => {}
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
        entities.create_component(component);
    }
}
