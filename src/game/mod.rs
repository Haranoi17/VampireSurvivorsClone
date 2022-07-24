use std::vec;

use graphics::{setup_textures, textures::GeneralTextureRepository};

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode},
};

fn clear_screen(window: &mut RenderWindow) {
    window.clear(Color::rgb(0, 0, 0));
}

struct Game {
    window: RenderWindow,
}

impl Game {
    pub fn initialize(&mut self) {
        self.create_window();
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

    fn create_window(&mut self) {
        let vide_mode = VideoMode {
            width: 1280,
            height: 720,
            ..Default::default()
        };

        self.window = RenderWindow::new(
            vide_mode,
            "test",
            Style::DEFAULT,
            &ContextSettings::default(),
        );
    }
}
