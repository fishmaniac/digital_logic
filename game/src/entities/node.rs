use std::{cell::RefCell, collections::{self, HashMap}, rc::Rc, sync::{atomic::{AtomicBool, Ordering::Relaxed}, Arc, Mutex, OnceLock}};

use engine::{ecs::{entity::{Entity, RectEntity}, entity_manager::EntityManager}, input::{GameEvent, Input}, renderer::Renderable, update::EventHandler, Canvas, Color, MouseButton, Rect, SdlWindow, Texture};

#[derive(PartialEq)]
enum NodeState {
    Unconnected,
    Connecting,
    Connected,
}

impl EventHandler for NodeConnection {
    fn handle_event(&mut self, event: &GameEvent, entities: &mut EntityManager) {
        match self.state {
            NodeState::Unconnected => {
            },
            NodeState::Connecting => {
            },
            NodeState::Connected => {
            },
        }
    }
}

static IS_CONNECTING: AtomicBool = AtomicBool::new(false);

pub struct NodeConnection {
    state: NodeState,
    pub connected_node: String,
    pub rect_entity: RectEntity,
    mouse_pos: (i32, i32),
}

impl NodeConnection {
    pub fn new(name: String, x: i32, y: i32, color: Color) -> Self {
        Self {
            state: NodeState::Unconnected,
            connected_node: "".to_string(),
            rect_entity: RectEntity::new(name, x, y, 2, 25, 25, color),
            mouse_pos: (100, 100),
        }
    }
    // pub fn check_connection(&mut self, event: &GameEvent, nodes: &mut [&mut NodeConnection]) {
    //     match event {
    //         GameEvent::MouseButtonDown { x, y, button: MouseButton::Left } => {
    //             if !IS_CONNECTING.load(Relaxed)
    //             && self.rect_entity.rect.contains_point((*x, *y)) {
    //                 IS_CONNECTING.store(true, Relaxed);
    //                 self.state = NodeState::Connecting;
    //                 println!("Connecting node");
    //             }
    //         },
    //         GameEvent::MouseButtonUp { x, y, button: MouseButton::Left } => {
    //             if self.state == NodeState::Unconnected
    //             && self.rect_entity.rect.contains_point((*x, *y))
    //             && IS_CONNECTING.load(Relaxed) {
    //                 // this will get stuck.. should check at last node
    //                 IS_CONNECTING.store(false, Relaxed); 
    //                 for node in nodes.iter_mut() {
    //                     if node.state == NodeState::Connecting {
    //                         self.connected_node = node.name().to_string();
    //                         self.mouse_pos = (node.rect_entity.x(), node.rect_entity.y());
    //                         self.state = NodeState::Connected;
    //
    //                         node.connected_node = self.name().to_string();
    //                         node.mouse_pos = (self.rect_entity.x(), self.rect_entity.y());
    //                         node.state = NodeState::Connected;
    //                     }
    //                 }
    //                 self.state = NodeState::Connected;
    //                 println!("set connection on: {}", self.name());
    //             }
    //
    //             if self.state == NodeState::Connecting {
    //                 self.state = NodeState::Unconnected;
    //             }
    //         },
    //         GameEvent::MouseMotion { x, y } => {
    //             if self.state == NodeState::Connecting {
    //                 self.mouse_pos = (*x, *y);
    //             }
    //         }
    //         _ => {},
    //     }
    // }
}

impl Renderable for NodeConnection {
    fn render(&self, canvas: &mut Canvas<SdlWindow>, textures: &HashMap<String, Texture>) {
        let rect = &self.rect_entity;
        rect.render(canvas, textures);
        match self.state {
            NodeState::Connecting | NodeState::Connected => {
                let node_pos = (rect.x(), rect.y());
                match canvas.draw_line(node_pos, self.mouse_pos) {
                    Ok(_) => {},
                    Err(e) => println!("Error drawing line: {}", e),
                }
            },
            _ => {},
        }
    }
}

impl Entity for NodeConnection {
    fn rect(&self) -> &Rect {
        &self.rect_entity.rect()
    }
    fn rect_mut(&mut self) -> &mut Rect {
        self.rect_entity.rect_mut()
    }
    fn name(&self) -> &str {
        self.rect_entity.name()
    }
    // fn as_updatable_mut(&mut self) -> Option<&mut dyn Updatable> {
    //     Some(self)
    // }
    fn as_event_handler_mut(&mut self) -> Option<&mut dyn EventHandler> {
        Some(self)
    }
    // fn as_texturable(&self) -> Option<&dyn Texturable> {
    //     Some(self)
    // }
}
