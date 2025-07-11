mod entities;

use engine::{
    ecs::entity::{RectEntity, TextureEntity},
    window::Window,
    Color,
    engine::Engine
};

use entities::and_gate::AndGate;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = engine::init()?;
    let window = match Window::new(&sdl_context) {
            Ok(window) => window,
            Err(e) => return Err(e),
    };
    let texture_creator = window.canvas.texture_creator();

    let mut engine = match Engine::new(sdl_context, window, &texture_creator) {
        Ok(engine) => engine,
        Err(e) => panic!("error creating engine {}", e),
    };

    let entity = RectEntity::new(
        "Entity1".to_string(),
        200,
        200,
        1,
        100,
        100,
        Color::RGB(255, 0, 0),
    );
    engine.entities.create_entity(
        &mut engine.renderer,
        entity
    );

    let entity = AndGate::new(
        "AndGate0".to_string(),
        200,
        200,
        Color::RGB(0, 0, 255),
    );
    engine.entities.create_entity(
        &mut engine.renderer,
        entity
    );

    let entity = AndGate::new(
        "AndGate1".to_string(),
        200,
        200,
        Color::RGB(0, 0, 255),
    );
    engine.entities.create_entity(
        &mut engine.renderer,
        entity
    );

    let entity = AndGate::new(
        "AndGate2".to_string(),
        200,
        200,
        Color::RGB(0, 0, 255),
    );
    engine.entities.create_entity(
        &mut engine.renderer,
        entity
    );

    engine.start();

    Ok(())
}
