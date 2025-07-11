use sdl3::{Sdl, video};
use sdl3::render::Canvas;
use sdl3::pixels::Color;

use std::error::Error;

pub struct Window {
    pub canvas: Canvas<video::Window>,
    background_color: Color,
}

impl Window {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Digital Logic", 800, 600)
            .position_centered()
            .resizable()
            .build()?;

        let mut canvas = window.into_canvas();

        let background_color = Color::RGB(100, 149, 237);
        canvas.set_draw_color(background_color);
        canvas.clear();
        canvas.present();

        Ok(Window {
            canvas,
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
}
