use sdl3::{
    event::Event, keyboard::Keycode, mouse::MouseButton, rect::Rect, EventPump, Sdl
};

use std::error::Error;

pub(crate) enum ExitStatus {
    Continue,
    Exit,
    Error(Box<dyn Error>),
}

pub enum InputEvent {
    KeyDown(Keycode),
    KeyUp(Keycode),
    MouseMotion { x: i32, y: i32 },
    MouseButtonDown { x: i32, y: i32, button: MouseButton },
    MouseButtonUp { x: i32, y: i32, button: MouseButton },
}

pub struct Input {
    event_pump: EventPump
}

impl Input {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let event_pump = sdl_context.event_pump()?;
        Ok(Input {
            event_pump,
        })
    }
    fn get_input(event: Event) -> (ExitStatus, Option<InputEvent>) {
        match event {
            Event::Quit { .. }
            | Event::KeyUp {
                keycode: Some(Keycode::Escape),
                ..
            } => (ExitStatus::Exit, None),
            Event::KeyDown { keycode: Some(key), .. } => {
                (ExitStatus::Continue, Some(InputEvent::KeyDown(key)))
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                (ExitStatus::Continue, Some(InputEvent::KeyUp(key)))
            }
            Event::MouseMotion { x, y, .. } => {
                (ExitStatus::Continue, Some(InputEvent::MouseMotion {
                    x: x as i32,
                    y: y as i32,
                }))
            }
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                (ExitStatus::Continue, Some(InputEvent::MouseButtonDown {
                    x: x as i32,
                    y: y as i32,
                    button: mouse_btn,
                }))
            }
            Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                (ExitStatus::Continue, Some(InputEvent::MouseButtonUp {
                    x: x as i32,
                    y: y as i32,
                    button: mouse_btn,
                }))
            }
            _ => (ExitStatus::Continue, None),
        }
    }
    pub(crate) fn poll_input(&mut self) -> (ExitStatus, Vec<InputEvent>) {
        let mut events = Vec::new();

        for event in self.event_pump.poll_iter() {
            let (status, maybe_event) = Self::get_input(event);

            if let Some(e) = maybe_event {
                events.push(e);
            }

            if let ExitStatus::Exit | ExitStatus::Error(_) = status {
                return (status, events);
            }
        }

        (ExitStatus::Continue, events)
    }
    pub fn rect_contains_mouse(rect: Rect, mouse_pos: (i32, i32)) -> bool {
        return rect.contains_point(mouse_pos)
    }
}
