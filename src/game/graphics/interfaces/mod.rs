use core::cmp::{Eq, PartialEq};
use sfml::graphics::Texture;

pub trait TextureRepository{
    fn get_texture(&self, name: TextureName) -> &Texture;
}

pub trait TextureConsumer<'a> {
    fn set_texture(&mut self, texture_repository: &'a impl TextureRepository);
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureName {
    Character,
}
