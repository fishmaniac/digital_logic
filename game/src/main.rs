use engine::{ecs::events::Event, engine::Engine};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = match Engine::new() {
        Ok(engine) => engine,
        Err(e) => return Err(e.into()),
    };


    let entity_id = engine.entities.create_rect_entity(0, 0, 100, 100);
    let entity_id_2 = engine.entities.create_rect_entity(300, 100, 50, 50);

    let entity_id_1 = engine.entities.create_entity();

    let entity = engine.entities.get_entity(entity_id).unwrap();
 
    match engine.start() {
        Ok(_) => {},
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
