use sdl3::{
    event::Event, keyboard::Keycode, mouse::MouseButton, rect::Rect, EventPump, Sdl
};

use std::{collections::HashSet, error::Error};

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
    event_pump: EventPump,
    pub pressed_keys: HashSet<Keycode>,
    pub pressed_mouse: HashSet<MouseButton>,
    pub position_mouse: (i32, i32),
}

impl Input {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let event_pump = sdl_context.event_pump()?;
        Ok(Input {
            event_pump,
            pressed_keys: HashSet::new(),
            pressed_mouse: HashSet::new(),
            position_mouse: (0, 0),
        })
    }
    fn input(event: Event) -> (ExitStatus, Option<InputEvent>) {
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
    pub fn keyboard_input(pressed_keys: &mut HashSet<Keycode>, event: &Option<InputEvent>) {
        if let Some(event) = event {
            match event {
                InputEvent::KeyDown(keycode) => {
                    pressed_keys.insert(*keycode);
                }
                InputEvent::KeyUp(keycode) => {
                    pressed_keys.remove(&keycode);
                }
                // InputEvent::MouseMotion { x, y } => self.mouse_position = (x, y),
                _ => {},
            }
        }
    }
    pub fn mouse_input(pressed_mouse: &mut HashSet<MouseButton>, mouse_position: &mut (i32, i32), event: &Option<InputEvent>) {
        if let Some(event) = event {
            match event {
                InputEvent::MouseMotion { x, y } => {
                    let (ref mut mouse_x, ref mut mouse_y) = *mouse_position;
                    *mouse_x = *x;
                    *mouse_y = *y;
                },
                InputEvent::MouseButtonDown { x, y, button } => {
                    pressed_mouse.insert(*button);
                },
                InputEvent::MouseButtonUp { x, y, button } => {
                    pressed_mouse.remove(&button);
                },
                _ => {},
            }
        }
    }

    pub(crate) fn poll_input(&mut self) -> ExitStatus {
        for event in self.event_pump.poll_iter() {
            let (status, event) = Self::input(event);
            let pressed_keys = &mut self.pressed_keys;
            Self::keyboard_input(pressed_keys, &event);
            let pressed_mouse = &mut self.pressed_mouse;
            let position_mouse = &mut self.position_mouse;
            Self::mouse_input(pressed_mouse, position_mouse, &event);

            if let ExitStatus::Exit | ExitStatus::Error(_) = status {
                return status;
            }
        }

        ExitStatus::Continue
    }
}
