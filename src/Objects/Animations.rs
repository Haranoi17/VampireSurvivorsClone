mod Tests;

use core::panic;
use sfml::{graphics::Texture, SfBox};
use std::clone;
use std::marker::Copy;
use std::{collections::HashMap, fs};

pub struct Animation {
    frames: Vec<SfBox<Texture>>,
}

pub enum PlayersAnimations {
    Idle,
}

impl PlayersAnimations {
    fn as_str(&self) -> &'static str {
        match self {
            PlayersAnimations::Idle => "idle",
        }
    }
}

impl Animation {
    pub fn new(animation_directory: &str) -> Self {
        let mut _frames: Vec<SfBox<Texture>> = Vec::new();

        let files = match fs::read_dir(animation_directory) {
            Ok(files) => files,
            Err(_) => {
                panic!("No such directory: {}", animation_directory)
            }
        };

        for file in files {
            let file_path: String = String::from(file.expect("no file").path().to_str().unwrap());
            _frames.push(Texture::from_file(file_path.as_str()).unwrap());
        }

        Self { frames: _frames }
    }

    pub fn get_frame_count(&self) -> usize {
        self.frames.len()
    }
}

pub struct AnimationPlayer {
    animations: HashMap<String, Animation>,

    current_animation: String,
    current_frame: usize,
    frames_per_second: usize,
    timer: f32,
}

impl AnimationPlayer {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: String::new(),
            current_frame: 0,
            frames_per_second: 2,
            timer: 0.0,
        }
    }

    pub fn initialize(&mut self, animations_path: String) {
        let animations_directories = match fs::read_dir(animations_path) {
            Ok(directory) => directory,
            Err(_) => {
                panic!("Players animations path is wrong!")
            }
        };

        for animation_directory in animations_directories {
            let animation_directory_entry = animation_directory.as_ref().unwrap();
            let animation_name = String::from(
                animation_directory
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
            );
            let animation_path: String =
                String::from(animation_directory_entry.path().to_str().unwrap());
            self.animations.insert(
                animation_name.clone(),
                Animation::new(animation_path.as_str()),
            );
            self.current_animation = animation_name.clone();
        }
    }

    pub fn get_current_animation_frame(&self) -> &Texture {
        &self.animations.get(&self.current_animation).unwrap().frames[self.current_frame]
    }

    pub fn set_current_animation(&mut self, animation_name: String) {
        self.current_animation = animation_name;
        self.timer = 0.0;
        self.current_frame = 0;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;

        self.current_frame = (self.timer * self.frames_per_second as f32) as usize;

        if self.current_frame >= self.current_animation_frame_count() {
            self.timer = 0.0;
            self.current_frame = 0;
            return;
        }
    }

    fn current_animation_frame_count(&self) -> usize {
        self.animations
            .get(&self.current_animation)
            .unwrap()
            .get_frame_count()
    }
}
