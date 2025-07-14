use std::{any::Any, collections::HashMap};

use engine::{ecs::{entity::{Draggable, Entity, RectEntity, Texturable, TextureEntity}, entity_manager::EntityManager}, input::GameEvent, renderer::Renderable, update::{DraggableEventHandler, EventHandler, Updatable}, Canvas, Color, Rect, SdlWindow, Texture};

use super::node::NodeConnection;

impl Updatable for AndGate {
    fn update(&mut self) {
        let gate_rect = self.texture_entity.rect();
        let node_x = gate_rect.x() - 10;
        let spacing_y = gate_rect.height() as i32 / 3;

        if let Some(in_a) = self.nodes.get_mut(0) {
            in_a.rect_entity.rect_mut().set_x(node_x);
            in_a.rect_entity.rect_mut().set_y(gate_rect.y() + spacing_y);
        }
        if let Some(in_b) = self.nodes.get_mut(1) {
            in_b.rect_entity.rect_mut().set_x(node_x);
            in_b.rect_entity.rect_mut().set_y(gate_rect.y() + 2 * spacing_y);
        }
        if let Some(out) = self.nodes.get_mut(2) {
            out.rect_entity.rect_mut().set_x(gate_rect.x() + gate_rect.width() as i32 - 10);
            out.rect_entity.rect_mut().set_y(gate_rect.y() + spacing_y);
        }
    }
}

impl EventHandler for AndGate {
    fn handle_event(&mut self, event: &GameEvent, entities: &mut EntityManager) {
        self.handle_drag_event(event);
        // let len = self.nodes.len();
        // for i in 0..len {
        //     let (left, right) = self.nodes.split_at_mut(i);
        //     let (node, right_rest) = right.split_first_mut().unwrap();
        //
        //     let mut mut_refs: Vec<&mut NodeConnection> =
        //         left.iter_mut().chain(right_rest.iter_mut()).collect();
        //
        //     node.check_connection(event, &mut mut_refs[..]);
        //     node.handle_event(event, entities);
        // }
    }
}

impl Draggable for AndGate {
    fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    fn set_dragging(&mut self, dragging: bool) {
        self.is_dragging = dragging;
    }
    fn drag_offset(&self) -> (i32, i32) {
        self.drag_offset
    }
    fn set_drag_offset(&mut self, offset: (i32, i32)) {
        self.drag_offset = offset
    }
}

pub struct AndGate {
    pub texture_entity: TextureEntity,
    is_dragging: bool,
    drag_offset: (i32, i32),
    nodes: Vec<NodeConnection>
}

impl AndGate {
    // TODO: move nodes up... (collision box system for entities?)
    pub fn new(name: String, x: i32, y: i32, color: Color) -> Self {
        let mut nodes = Vec::new();
        let gate = RectEntity::new(name.clone(), x, y, 1, 100, 100, color);
        let in_a = NodeConnection::new(name.clone() + "_node_a", x, y, Color::RGB(100, 20, 20));
        let in_b = NodeConnection::new(name.clone() + "_node_b", x, y, Color::RGB(100, 20, 20));
        let out = NodeConnection::new(name + "_node_out", x, y, Color::RGB(20, 100, 20));

        nodes.push(in_a);
        nodes.push(in_b);
        nodes.push(out);
        AndGate {
            texture_entity: TextureEntity::new(gate, "assets/and.bmp".to_string()),
            is_dragging: false,
            drag_offset: (0, 0),
            nodes,
        }
    }
}

impl Texturable for AndGate {
    fn texture_name(&self) -> &String {
        &self.texture_entity.texture_name
    }
    fn set_texture_name(&mut self, name: String) {
        self.texture_entity.texture_name = name
    }
}

impl Renderable for AndGate {
    fn render(&self, canvas: &mut Canvas<SdlWindow>, textures: &HashMap<String, Texture>) {
        self.texture_entity.render(canvas, textures);
        for node in &self.nodes {
            node.render(canvas, textures);
        }
    }
    fn z(&self) -> u32 {
        self.texture_entity.z()
    }
}

impl Entity for AndGate {
    fn rect(&self) -> &Rect {
        &self.texture_entity.rect()
    }
    fn rect_mut(&mut self) -> &mut Rect {
        self.texture_entity.rect_mut()
    }
    fn name(&self) -> &str {
        self.texture_entity.name()
    }
    fn as_updatable_mut(&mut self) -> Option<&mut dyn Updatable> {
        Some(self)
    }
    fn as_event_handler_mut(&mut self) -> Option<&mut dyn EventHandler> {
        Some(self)
    }
    fn as_texturable(&self) -> Option<&dyn Texturable> {
        Some(self)
    }
}
