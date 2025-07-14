use sdl3::{pixels::Color, rect::Rect as SdlRect};

use super::{entity::Entity, events::Event};

#[derive(Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self{
        Self {
            x,
            y,
        }
    }
    pub fn update(&mut self) {
        println!("Updating position");
    }
    pub fn movement_callback(&mut self, event: &Event) {
        match event {
            Event::Position(x, y) => {
                self.x += x;
                self.y += y;
            },
            Event::LeftClick(_, _) => todo!(),
        }
        println!("Movement listener {}, {}", self.x, self.y);
    }
}

pub(crate) struct Rect {
    pub rect: SdlRect,
    pub color: Color,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: SdlRect::new(x, y, width, height),
            color: Color::RGB(0, 255, 255),
        }
    }
    pub fn update(&mut self) {
        println!("Update rect");
    }
    pub fn set_position(&mut self, position: Position) {
        self.rect.set_x(position.x);
        self.rect.set_y(position.y);
    }
    pub fn mouse_contains(&mut self, x: i32, y: i32) -> bool {
        false
    }
}

pub enum EngineComponent {
    Position(Position),
    Rect(Rect),
    Draggable,
}

impl EngineComponent {
    pub fn update(&mut self) {
        match self {
            EngineComponent::Position(position) => position.update(),
            EngineComponent::Rect(rect) => rect.update(),
            EngineComponent::Draggable => {},
        }
    }
}

trait GameComponent {
    fn update(&mut self, entity: &Entity);
}

pub struct Components {
    pub engine_components: Vec<EngineComponent>,
    pub game_components: Vec<Box<dyn GameComponent>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            engine_components: Vec::new(),
            game_components: Vec::new(),
        }
    }
}
