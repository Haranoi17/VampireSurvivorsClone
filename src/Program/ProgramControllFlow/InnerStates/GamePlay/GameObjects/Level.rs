use std::vec;

use sfml::graphics::RenderWindow;

use crate::{
    CollisionSystem::{Collidable, Symulation::WordSymulation},
    InputSystem::{Input, InputConsumer},
    MathUtilities::Position,
    Objects::{
        Interfaces::{Drawable, Initializable, Updatable},
        Timers::{BasicTimer, Timer},
    },
};

use super::{
    Enemy::{Enemy, Spawner::{Spawner, WaveSpawner}},
    Player::Player,
};

use serde::{Deserialize, Serialize};



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LevelConfiguration {
    pub name: String,
    pub waves: Vec<Wave>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Wave{
    pub enemy_count: usize,
    pub enemy_spawn_delay_in_seconds: f32,
}

pub struct Level {
    player: Player,
    enemies: Vec<Enemy>,

    collision_symulation: WordSymulation,
    wave_spawner: WaveSpawner,
    
    configuration: LevelConfiguration,
    current_wave: usize,
}

impl Level {
    pub fn new(level_configuration: LevelConfiguration)->Self{
        let mut new_level = Self {
            player: Player::new(),
            enemies: vec![],
            collision_symulation: WordSymulation::new(),
            wave_spawner: WaveSpawner::new(level_configuration.waves.get(0).unwrap().clone()),
            configuration: level_configuration,
            current_wave: 0
        };
        new_level.initialize();
        new_level
    }

    fn update_player(&mut self, delta_time: f32) {
        self.player.update(delta_time);
    }

    fn update_enemies(&mut self, delta_time: f32) {
        for enemy in &mut self.enemies {
            enemy.update(delta_time);
            enemy.walk_towards(self.player.get_position(), delta_time);
        }
    }

    fn update_wave_spawner(&mut self, delta_time: f32) {
        if self.wave_spawner.finished_spawning(){
            self.current_wave += 1usize;

            let new_wave = self.configuration.waves.get(self.current_wave);
            match new_wave {
                Some(wave) =>{ 
                    self.wave_spawner = WaveSpawner::new(wave.clone());
                    self.wave_spawner.start_spawning();
                }
                
                None => self.wave_spawner.stop_spawning(),
            }          
        }

        self.wave_spawner.update(delta_time);
        if self.wave_spawner.should_spawn() {
            self.enemies.extend(self.wave_spawner.spawn());
            self.wave_spawner.start_spawning();
        }
    }

    fn update_word(&mut self, delta_time: f32) {
        let mut collidables: Vec<&mut dyn Collidable> = vec![];
        for enemy in &mut self.enemies {
            collidables.push(enemy);
        }

        collidables.push(&mut self.player);

        self.collision_symulation.collision_detection(&collidables);
        self.collision_symulation
            .react_to_collisionss(&mut collidables);
        self.collision_symulation.clear_collisions();
        collidables.clear();
    }

    fn draw_player(&mut self, window: &mut RenderWindow) {
        self.player.draw(window);
    }

    fn draw_enemies(&mut self, window: &mut RenderWindow) {
        for enemy in &mut self.enemies {
            enemy.draw(window);
        }
    }
}

impl Updatable for Level {
    fn update(&mut self, delta_time: f32) {
        self.update_player(delta_time);
        self.update_wave_spawner(delta_time);
        self.update_enemies(delta_time);
        self.update_word(delta_time);
    }
}

impl Drawable for Level {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.draw_player(window);
        self.draw_enemies(window);
    }
}

impl InputConsumer for Level {
    fn handle_input(&mut self, input: &Input) {
        self.player.handle_input(input);
    }
}

impl Initializable for Level {
    fn initialize(&mut self) {
        self.player.initialize();
        self.wave_spawner.start_spawning();
    }
}
