use std::time::Instant;
use std::{error::Error, time::Duration};

use sdl3::{render::TextureCreator, video::WindowContext, Sdl};

use crate::input::{ExitStatus, GameEvent};
use crate::{
    input::Input, renderer::Renderer, window::Window
};
use crate::ecs::entity_manager::EntityManager;

pub struct Engine <'t> {
    pub window: Window,
    pub input: Input,
    pub entities: EntityManager,
    pub renderer: Renderer<'t>,
}
impl <'t> Engine <'t> {
    pub fn new(sdl: Sdl, window: Window, texture_creator: &'t TextureCreator<WindowContext>) -> Result<Self, String> {
        let input = match Input::new(&sdl) {
            Ok(input) => input,
            Err(e) => return Err(e.to_string()),
        };
        let entities = EntityManager::new();
        let renderer = Renderer::new(&texture_creator);

        Ok(Self {
            window,
            input,
            entities,
            renderer,
        })
    }

    pub fn start(&mut self) {
        let mut last_time = Instant::now();
        let mut fps_timer = Instant::now();
        let mut frame_count = 0;

        loop {
            let (exit_status, game_events) = self.input.poll_events();

            self.events(game_events);
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
                ExitStatus::_Error(e) => panic!("Exit status error: {}", e),
            }

            let frame_duration = Duration::from_secs_f64(1.0 / 30.0);
            let elapsed = last_time.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
            last_time = Instant::now();
        }
    }

    // pub fn start(&mut self) {
    //     loop {
    //         let (exit_status, game_events) = self.input.poll_events();
    //
    //         self.events(game_events);
    //         self.update();
    //         self.render();
    //
    //         match exit_status {
    //             ExitStatus::Continue => {},
    //             ExitStatus::Exit => break,
    //             ExitStatus::_Error(e) => panic!("Exit status error: {}", e),
    //         }
    //     }
    // }

    fn events(&mut self, events: Vec<GameEvent>) {
        self.entities.handle_events(events);
    }

    fn update(&mut self) {
        self.entities.update_entities();
    }

    fn render(&mut self) {
        self.window.clear();
        self.renderer.render(&mut self.window.canvas);
        self.window.present();
    }
}
