use sdl3::{
    render::{Canvas, ScaleMode, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext}
};

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use std::collections::HashMap;

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>, textures: &HashMap<String, Texture>);
    fn z(&self) -> u32 {
        0
    }
}

pub struct Renderer<'t> {
    renderables: BTreeMap<u32, Vec<Rc<RefCell<dyn Renderable>>>>,
    textures: HashMap<String, Texture<'t>>,
    texture_creator: &'t TextureCreator<WindowContext>,
}

impl<'t> Renderer<'t> {
    pub fn new(texture_creator: &'t TextureCreator<WindowContext>) -> Self {
        Renderer {
            renderables: BTreeMap::new(),
            textures: HashMap::new(),
            texture_creator,
        }
    }

    pub fn add_renderable(&mut self, renderable: Rc<RefCell<dyn Renderable>>) {
        let z = renderable.borrow().z();
        self.renderables.entry(z).or_default().push(renderable);
    }

    pub fn add_texture(&mut self, texture_path: String) {
        if self.textures.contains_key(&texture_path) {
            return;
        }
        let surface = Surface::load_bmp(Path::new(&texture_path))
            .expect("Failed to load BMP");
        let mut texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture");
        texture.set_scale_mode(ScaleMode::Nearest);

        self.textures.insert(texture_path, texture);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for (_z, items) in &self.renderables {
            for item in items {
                item.borrow().render(canvas, &self.textures);
            }
        }
    }
}
