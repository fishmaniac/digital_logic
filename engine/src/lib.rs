pub mod engine;
pub mod input;
pub mod renderer;
pub mod window;
pub mod update;
pub mod ecs;

pub use sdl3::{
    init,
    pixels::Color,
    render::{Canvas, Texture},
    video::Window as SdlWindow,
    keyboard::Keycode,
    mouse::MouseButton,
    rect::Rect,
};

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
