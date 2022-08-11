use sfml::graphics::RenderWindow;

use crate::{Objects::Interfaces::{Drawable, Updatable}, InputSystem::InputConsumer};

use self::GameObjects::Level::Level;

mod GameObjects;

pub struct GamePlayInnerState {
    level: Level
}

impl GamePlayInnerState {
    pub fn new() -> Self {
        Self {level: Level::new()}
    }

    fn update_level(&mut self, delta_time: f32){
        self.level.update(delta_time);
    }
    
    fn draw_level(&mut self, window: &mut RenderWindow){
        self.level.draw(window);
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

impl InputConsumer for GamePlayInnerState{
    fn handle_input(&mut self, input: &crate::InputSystem::Input) {
        self.level.handle_input(input);
    }
}