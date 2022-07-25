use std::vec;

pub mod graphics;
use graphics::{
    interfaces::{TextureConsumer, TextureRepository},
    textures::GeneralTextureRepository,
};

mod objects;
use objects::entities::Entity;

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode},
};

use self::graphics::interfaces::TextureName;

fn clear_screen(window: &mut RenderWindow) {
    window.clear(Color::rgb(0, 0, 0));
}

pub struct Game<'a, 't:'a, T: TextureRepository> {
    window: RenderWindow,
    texture_repository: &'t T,
    drawables: Vec<Entity<'a>>,
}

impl<'a, 't:'a, T: TextureRepository> Game<'a, 't, T>{
    pub fn new(texture_repository:  &'t T ) -> Self {
        let vide_mode = VideoMode {
            width: 1280,
            height: 720,
            ..Default::default()
        };

        Self {
            window: RenderWindow::new(
                vide_mode,
                "test",
                Style::DEFAULT,
                &ContextSettings::default(),
            ),
            texture_repository: texture_repository,
            drawables: vec![Entity::new()],
        }
    }

    pub fn initialize(&mut self) {
        for drawable in &mut self.drawables {
            drawable.set_texture(self.texture_repository.get_texture(TextureName::Character));
        }
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                _ => { /* Do nothing */ }
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {}

    pub fn is_on(&self) -> bool{
        self.window.is_open()
    }
}
