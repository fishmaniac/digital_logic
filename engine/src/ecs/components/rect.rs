use sdl3::{pixels::Color, rect::Rect as SdlRect, render::Canvas, video::Window};

use super::ColorRGB;
use super::{
    super::components::{ComponentStorage, EngineComponent},
    position::Position,
};
use crate::ecs::events::{EntityEvent, Events};
use crate::{
    ecs::{entities::Entities, events::Event},
    renderer::Renderable,
};

#[derive(Debug)]
pub struct Rect {
    pub rect: SdlRect,
    pub color: Color,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: ColorRGB) -> Self {
        Self {
            rect: SdlRect::new(x, y, width, height),
            color: color.into(),
        }
    }
    fn position(&mut self, position: Position) {
        self.rect.set_x(position.x());
        self.rect.set_y(position.y());
    }
    fn update_position(entities: &mut Entities, entity_id: u32) {
        let position = entities.get_component_mut::<Position>(entity_id);
        let position = match position {
            Some(position) => position.clone(),
            None => return println!("No position for update_position entity_id {}", entity_id),
        };

        let rect = entities.get_component_mut::<Rect>(entity_id);
        match rect {
            Some(rect) => rect.position(position),
            None => return println!("No rect for update_position entity_id {}", entity_id),
        };
    }
    pub fn contains(&mut self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }
    pub fn x(&self) -> i32 {
        self.rect.x
    }
    pub fn y(&self) -> i32 {
        self.rect.y
    }
}

impl Renderable for Rect {
    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.rect);
    }
}

impl ComponentStorage for Rect {
    fn get_mut(entities: &mut Entities, entity_id: u32) -> Option<&mut Self> {
        entities
            .rect_components
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
            EntityEvent::Position(_, _) => Rect::update_position(entities, entity_id),
            _ => {}
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent, events: &mut Events) {
        match &component {
            EngineComponent::Rect(rect) => {
                let position = EngineComponent::Position(Position::new(rect.rect.x, rect.rect.y));
                entities.create_engine_component::<Position>(events, position);
            }
            _ => println!("Invalid component for Rect"),
        };
        entities.create_component(component);
    }
}
