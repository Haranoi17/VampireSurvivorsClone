use sfml::{
    graphics::{RenderTarget, Sprite, Texture, Transformable, Text},
    system::Vector2f,
    SfBox,
};

use crate::game::graphics::interfaces::{TextureConsumer, TextureName, TextureRepository};

pub struct Entity<'a> {
    position: Vector2f,
    speed: f32,

    sprite: Sprite<'a>,
}

impl Entity<'_> {
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

impl<'b, 't:'b> TextureConsumer<'t> for Entity<'b> {
    fn set_texture<'a>(&'a mut self, texture: &'t SfBox<Texture>) where 't:'a{
        self.sprite.set_texture(  &texture, false);
        self.sprite.set_scale((0.1, 0.1));
    }
}
