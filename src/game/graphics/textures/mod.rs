use sfml::{graphics::Texture, SfBox};

use std::collections::HashMap;
use super::interfaces::{TextureName, TextureRepository};

pub struct GeneralTextureRepository {
    textures: HashMap<TextureName, SfBox<Texture>>,
}

impl GeneralTextureRepository {
    fn load_texture(file_path: &str) -> SfBox<Texture> {
        match Texture::from_file(file_path) {
            Some(texture_box) => texture_box,
            None => {
                panic!("No image")
            }
        }
    }

    pub fn new() -> Self {
        let mut new_repository = Self {
            textures: HashMap::new(),
        };

        new_repository.textures.insert(
            TextureName::Character,
            GeneralTextureRepository::load_texture("resources/character.png"),
        );

        new_repository
    }
}

impl TextureRepository for GeneralTextureRepository{
    fn get_texture(&self, name: TextureName) -> &Texture {
        self.textures.get(&name).unwrap()
    }
}