use std::{fs::File, path::Path};

use sfml::graphics::RenderWindow;

use crate::{
    InputSystem::InputConsumer,
    Objects::Interfaces::{Drawable, Updatable},
};

use self::GameObjects::Level::{Level, LevelConfiguration};

mod GameObjects;

pub struct GamePlayInnerState {
    currently_loaded_level: Level,
}

impl GamePlayInnerState {
    pub fn new() -> Self {
        let file_path = Path::new("resources/GameplayConfig/Levels/Level_1.json");
        let file =File::open(file_path).unwrap();
        let level_config: LevelConfiguration = serde_json::from_reader(file).expect("error parsing file");
        Self {
            currently_loaded_level: Level::new(level_config),
        }
    }

    fn update_level(&mut self, delta_time: f32) {
        self.currently_loaded_level.update(delta_time);
    }

    fn draw_level(&mut self, window: &mut RenderWindow) {
        self.currently_loaded_level.draw(window);
    }
}

impl Updatable for GamePlayInnerState {
    fn update(&mut self, delta_time: f32) {
        self.update_level(delta_time);
    }
}

impl Drawable for GamePlayInnerState {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.draw_level(window);
    }
}

impl InputConsumer for GamePlayInnerState {
    fn handle_input(&mut self, input: &crate::InputSystem::Input) {
        self.currently_loaded_level.handle_input(input);
    }
}
