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
    pub fn movement_callback(&mut self, event: &Event) {
        match event {
            Event::Position(x, y) => {
                self.x += x;
                self.y += y;
            },
            Event::LeftClick(_, _) => todo!(),
        }
        // println!("Movement listener {}, {}", self.x, self.y);
    }
}

pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}
impl From<ColorRGB> for Color {
    fn from(rgb: ColorRGB) -> Self {
        Color::RGB(rgb.r, rgb.g, rgb.b)
    }
}

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
    pub fn set_position(&mut self, position: Position) {
        self.rect.set_x(position.x);
        self.rect.set_y(position.y);
    }
    pub fn mouse_contains(&mut self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }
}

pub enum EngineComponent {
    Position(Position),
    Rect(Rect),
    Draggable,
}

impl EngineComponent {
    // pub fn update(&mut self) {
    //     match self {
    //         EngineComponent::Position(position) => position.update(),
    //         EngineComponent::Rect(rect) => rect.update(),
    //         EngineComponent::Draggable => {},
    //     }
    // }
}

pub trait GameComponent {
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
