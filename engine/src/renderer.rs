use sdl3::video::{Window, WindowContext};
use sdl3::Sdl;
use sdl3::render::{Canvas, TextureCreator};
use sdl3::pixels::Color;

use std::error::Error;

use crate::ecs::components::EngineComponent;
use crate::ecs::entities::Entities;

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>);
}

pub struct Renderer {
    pub canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    background_color: Color,
}

impl Renderer {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Digital Logic", 800, 600)
            .position_centered()
            .resizable()
            .build()?;

        let mut canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        let background_color = Color::RGB(100, 149, 237);
        canvas.set_draw_color(background_color);
        canvas.clear();
        canvas.present();

        Ok(Renderer {
            canvas,
            texture_creator,
            background_color,
        })
    }
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
    pub fn render(&mut self, entities: &Entities) {
        entities.render(&mut self.canvas);
    }
}
