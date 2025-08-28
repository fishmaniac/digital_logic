use engine::{
    create_entity,
    ecs::components::{ColorRGB, ComponentStorage, EngineComponent, rect::Rect},
    engine::Engine,
};
use entities::and::AndGate;

use crate::global_state::GlobalState;

mod entities;
mod global_state;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let global_state = GlobalState::new();
    let mut engine = match Engine::new(global_state) {
        Ok(engine) => engine,
        Err(e) => return Err(e.into()),
    };

    for i in 0..1000 {
        let x = (i * 15) % 1920 as i32;
        let y = (i * 10) % 1080 as i32;

        let width = (10 + (i * 7) % 90) as u32;
        let height = (10 + (i * 5) % 90) as u32;

        let r = ((i * 3) % 256) as u8;
        let g = ((i * 5) % 256) as u8;
        let b = ((i * 7) % 256) as u8;

        let rect = Rect::new(x, y, width, height, ColorRGB::new(r, g, b));
        let id = create_entity!(
            &mut engine.entities,
            &mut engine.events,
            (Rect, EngineComponent::Rect(rect)),
        );
    }
    let id = create_entity!(
        &mut engine.entities,
        &mut engine.events,
        (
            Rect,
            EngineComponent::Rect(Rect::new(300, 400, 100, 100, ColorRGB::new(0, 255, 0)))
        ),
    );
    engine.events.add_entity_listener(id, Rect::entity_listener);

    let id = engine.entities.create_entity();
    println!("Game Entity id: {:?}", id);

    let entity = engine.entities.get_entity(id).unwrap();

    let and = AndGate::new(&mut engine.entities, &mut engine.events);
    let and = AndGate::new(&mut engine.entities, &mut engine.events);

    match engine.start() {
        Ok(_) => {}
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
