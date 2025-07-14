use std::time::{Duration, Instant};

use sdl3::init;

use crate::{
    ecs::{entities::Entities, events::Events},
    input::{ExitStatus, InputEvent, Input},
    renderer::Renderer
};

pub struct Engine {
    pub renderer: Renderer,
    pub input: Input,
    pub entities: Entities,
    pub events: Events,
}
impl Engine {
    pub fn new() -> Result<Self, String> {
        let sdl = match init() {
            Ok(sdl) => sdl,
            Err(e) => return Err(e.to_string()),
        };
        let renderer = match Renderer::new(&sdl) {
            Ok(window) => window,
            Err(e) => return Err(e.to_string()),
        };
        let input = match Input::new(&sdl) {
            Ok(input) => input,
            Err(e) => return Err(e.to_string()),
        };
        let mut entities = Entities::new();
        let events = Events::new(&mut entities);

        Ok(Self {
            renderer,
            input,
            entities,
            events,
        })
    }

    pub fn start(&mut self) -> Result<(), String>{
        let mut last_time = Instant::now();
        let mut fps_timer = Instant::now();
        let mut frame_count = 0;

        loop {
            let exit_status = self.input.poll_input();

            self.events();
            self.update();
            self.render();

            frame_count += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                let fps = frame_count as f64 / fps_timer.elapsed().as_secs_f64();
                println!("FPS: {:.2}", fps);
                frame_count = 0;
                fps_timer = Instant::now();
            }

            match exit_status {
                ExitStatus::Continue => {},
                ExitStatus::Exit => break,
                ExitStatus::Error(e) => return Err(e.to_string()),
            }

            let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
            let elapsed = last_time.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
            last_time = Instant::now();
        }
        Ok(())
    }

    fn events(&mut self) {
        let events = self.events.handle_input_events(&self.input);
        for event in events {
            self.events.handle_callback(&mut self.entities, event);
        }
    }

    fn update(&mut self) {
        self.entities.update();
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.renderer.render(&self.entities);
        self.renderer.present();
    }
}
