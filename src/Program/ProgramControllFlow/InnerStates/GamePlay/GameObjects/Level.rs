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
    Enemy::{Enemy, Spawner::Spawner},
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
    pub enemy_spawn_delay_in_seconds: usize,
}

pub struct Level {
    player: Player,
    enemies: Vec<Enemy>,

    collision_symulation: WordSymulation,
    enemy_spawner: Spawner,
    
    configuration: LevelConfiguration,
    current_wave: Wave,
}

impl Level {
    pub fn new(level_configuration: LevelConfiguration)->Self{
        let mut new_level = Self {
            player: Player::new(),
            enemies: vec![],
            collision_symulation: WordSymulation::new(),
            enemy_spawner: Spawner::new(0.0),
            configuration: level_configuration,
            current_wave: level_configuration.waves.clone().get(0).unwrap().to_owned(),
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

    fn update_enemy_spawner(&mut self, delta_time: f32) {
        self.enemy_spawner.update(delta_time);
        if self.enemy_spawner.should_spawn() {
            self.enemies.extend(self.enemy_spawner.spawn());
            self.enemy_spawner.start_spawning();
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
        self.update_enemies(delta_time);
        self.update_word(delta_time);
        self.update_enemy_spawner(delta_time);
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

        self.enemy_spawner.start_spawning();
    }
}
