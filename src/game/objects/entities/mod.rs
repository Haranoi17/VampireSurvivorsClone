use sfml::{
    graphics::{RenderTarget, Sprite, Texture, Transformable},
    system::Vector2f,
};

use crate::game::graphics::interfaces::{TextureConsumer, TextureName, TextureRepository};

pub struct Entity<'a> {
    position: Vector2f,
    speed: f32,

    sprite: Sprite<'a>,
}

impl<'a> Entity<'a> {
    pub fn new() -> Self {
        Self {
            position: Vector2f::new(0.0, 0.0),
            speed: 1.0,
            sprite: Sprite::new(),
        }
    }

    pub fn walk(&mut self, direction: Vector2f) {
        self.position += direction * self.speed;
    }

    pub fn draw(&self, window: &mut sfml::graphics::RenderWindow) {
        window.draw(&self.sprite);
    }
}

impl<'a> TextureConsumer<'a> for Entity<'a>{
    fn set_texture(&mut self, texture_repository: &'a impl TextureRepository) {
        let texture: &Texture = texture_repository.get_texture(TextureName::Character );
        self.sprite.set_texture(texture, false);
        self.sprite.set_scale((0.1, 0.1));
    }
}
