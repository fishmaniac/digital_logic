use sdl3::{
    pixels::Color,
    rect::Rect as SdlRect,
    render::Canvas,
    video::Window
};

use crate::{
    ecs::{entities::Entities, events::Event},
    renderer::Renderable
};
use super::{
    super::components::{ComponentStorage, EngineComponent},
    position::Position,
};
use super::ColorRGB;

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
    pub fn set_position(entities: &mut Entities, entity_id: u32) {
        let position = entities.get_component_mut::<Position>(entity_id);
        let position = match position {
            Some(position) => position.clone(),
            None => return println!("No position for set_position entity_id {}", entity_id),
        };

        let rect = entities.get_component_mut::<Rect>(entity_id);
        let rect = match rect {
            Some(rect) => rect.position(position),
            None => return println!("No rect for set_rect entity_id {}", entity_id),
        };
    }
    pub fn contains(&mut self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
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
        entities.rect_components
            .get_mut(entity_id as usize).and_then(|position| position.as_mut())
    }
    fn listener(entities: &mut Entities, entity_id: u32, event: &Event) {
        match event {
            Event::ComponentUpdate => {},
            Event::StateUpdate => {}
            Event::Position(_, _) => {
                let mut position = entities.get_component_mut::<Position>(entity_id);
                match position {
                    Some(ref mut position) => position.movement_callback(&event),
                    None => (),
                }
                let position = position.cloned();
                let rect = entities.get_component_mut::<Rect>(entity_id);
                match rect {
                    Some(rect) => {
                        if let Some(position) = position {
                            // TODO: use set_position instead
                            // remove temp movement callback updates from Position
                            rect.position(position);
                        }
                    }
                    None => (),
                }
            },
            Event::LeftClick(_, _) => {},
        }
    }
    fn create(entities: &mut Entities, component: EngineComponent) {
        match &component {
            EngineComponent::Rect(rect) => {
                entities.create_component(EngineComponent::Position(Position::new(
                    rect.rect.x,
                    rect.rect.y
                )));
            },
            _ => println!("Invalid component for Rect"),
        };
        entities.create_component(component);
        // entities.create_entity()
    }
}
