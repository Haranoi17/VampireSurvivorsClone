use core::cmp::{Eq, PartialEq};
use sfml::graphics::Texture;
use sfml::SfBox;

pub trait TextureRepository{
    fn get_texture(&self, name: TextureName) -> &SfBox<Texture>;
}

pub trait TextureConsumer{
    fn set_texture<'a, 't:'a>(&'a mut self, texture: &'t SfBox<Texture>);
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureName {
    Character,
}
