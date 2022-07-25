use core::cmp::{Eq, PartialEq};
use sfml::graphics::Texture;
use sfml::SfBox;

pub trait TextureRepository{
    fn get_texture(&self, name: TextureName) -> &SfBox<Texture>;
}

pub trait TextureConsumer<'t>{
    fn set_texture<'a>(&'a mut self, texture: &'t SfBox<Texture>) where 't:'a;
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureName {
    Character,
}
