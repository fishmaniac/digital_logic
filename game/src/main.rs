use engine::{
    ecs::{components::{ColorRGB, EngineComponent, Rect}, entities::ComponentStorage, events::Event},
    engine::Engine
};
use entities::and::AndGate;

mod entities;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = match Engine::new() {
        Ok(engine) => engine,
        Err(e) => return Err(e.into()),
    };

    // for i in 0..100 {
    //     let entity_id = engine.entities.create::<Rect>(EngineComponent::Rect(Rect::new(100, 100, 100, 100, ColorRGB::new(255, 0, 0))));
    // }
    let entity_id = engine.entities.create::<Rect>(EngineComponent::Rect(Rect::new(300, 400, 100, 100, ColorRGB::new(0, 255, 0))));
    engine.events.add_listener(entity_id, Rect::listener);

    let entity_id = engine.entities.create_entity();

    let entity = engine.entities.get_entity(entity_id).unwrap();

    let and = AndGate::new(&mut engine.entities, &mut engine.events);
 
    match engine.start() {
        Ok(_) => {},
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
