use std::vec::Vec;

pub mod textures;
pub mod interfaces;
use interfaces::{TextureConsumer, TextureRepository};

pub fn setup_textures<'a>(texture_consumers: &mut Vec<impl TextureConsumer<'a>>, texture_repository: &'a impl TextureRepository){
    for texture_consumer in texture_consumers{
        texture_consumer.set_texture(texture_repository);
    }
}