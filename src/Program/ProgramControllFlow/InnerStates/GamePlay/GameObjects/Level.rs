use std::{ops::Index, vec};

use sfml::graphics::RenderWindow;

use crate::{
    CollisionSystem::{Collidable, Symulation::WordSymulation},
    InputSystem::{Input, InputConsumer},
    MathUtilities::Position,
    Objects::{
        Interfaces::{Destroyable, Drawable, Initializable, Updatable},
        Timers::{BasicTimer, Timer},
    },
};

use super::{
    Spawner::Spawner,
    Weapons::{self, SimpleMissile::SimpleMissile},
};

use super::{
    Enemy::{Enemy, EnemySpawners::WaveSpawner},
    Player::{Player, WeaponSpawners::SimpleMissileSpawner},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LevelConfiguration {
    pub name: String,
    pub waves: Vec<Wave>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Wave {
    pub enemy_count: usize,
    pub enemy_spawn_delay_in_seconds: f32,
}

pub struct Level {
    player: Player,
    weapon_spawner: SimpleMissileSpawner,

    enemies: Vec<Enemy>,
    weapons: Vec<SimpleMissile>,

    collision_symulation: WordSymulation,
    wave_spawner: WaveSpawner,

    configuration: LevelConfiguration,
    current_wave: usize,
}

impl Level {
    pub fn new(level_configuration: LevelConfiguration) -> Self {
        let mut new_level = Self {
            player: Player::new(),
            enemies: vec![],
            weapons: vec![],
            collision_symulation: WordSymulation::new(),
            wave_spawner: WaveSpawner::new(level_configuration.waves.get(0).unwrap().clone()),
            weapon_spawner: SimpleMissileSpawner::new(0.2),
            configuration: level_configuration,
            current_wave: 0,
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

        Level::delete_objects_ready_to_destroy(&mut self.enemies);
    }

    fn update_weapons(&mut self, delta_time: f32) {
        for weapon in &mut self.weapons {
            weapon.update(delta_time);
        }

       Level::delete_objects_ready_to_destroy(&mut self.weapons);
    }

    fn delete_objects_ready_to_destroy<T>(destroyables: &mut Vec<T>) where T: Destroyable{
        let to_delete = Level::get_indices_of_objects_to_delete(&destroyables);
        for i in 0..to_delete.len() {
            //after removing element rest of them are being shifted by one to the left so next element to remove is its index minus removed elements count
            destroyables.remove(to_delete.get(i).unwrap() - i);
        }
    }

    fn get_indices_of_objects_to_delete<T>(destroyables: &Vec<T>) -> Vec<usize> where T: Destroyable{
        let mut to_delete: Vec<usize> = vec![];
        for i in 0..destroyables.len() {
            if destroyables.get(i).unwrap().should_be_destroyed() {
                to_delete.push(i);
            }
        }

        to_delete
    }
    
    fn update_wave_spawner(&mut self, delta_time: f32) {
        self.wave_spawner.update(delta_time);

        if self.wave_spawner.finished_spawning() {
            self.current_wave += 1usize;

            let new_wave = self.configuration.waves.get(self.current_wave);
            match new_wave {
                Some(wave) => {
                    self.wave_spawner = WaveSpawner::new(wave.clone());
                    self.wave_spawner.start_spawning();
                }

                None => self.wave_spawner.stop_spawning(),
            }
        }

        if self.wave_spawner.should_spawn() {
            self.enemies.extend(self.wave_spawner.spawn());
            self.wave_spawner.start_spawning();
        }
    }

    fn update_weapon_spawners(&mut self, delta_time: f32) {
        self.weapon_spawner.update(delta_time);

        if self.enemies.is_empty() || !self.weapon_spawner.should_spawn() {
            return;
        }

        let mut min_distance_enemy_position = self.enemies.get(0).unwrap().get_collider().position;
        for enemy in &self.enemies {
            let distance_from_enemy =
                (enemy.get_collider().position - self.player.get_position()).length();
            let min_distance = (min_distance_enemy_position - self.player.get_position()).length();
            if distance_from_enemy < min_distance {
                min_distance_enemy_position = enemy.get_collider().position;
            }
        }
        self.weapon_spawner
            .set_spawn_position(self.player.get_position());
        self.weapon_spawner
            .set_target_position(min_distance_enemy_position);
        self.weapons.extend(self.weapon_spawner.spawn());
        self.weapon_spawner.start_spawning();
    }

    fn update_word(&mut self, delta_time: f32) {
        let mut collidables: Vec<&mut dyn Collidable> = vec![];
        for enemy in &mut self.enemies {
            collidables.push(enemy);
        }

        for weapon in &mut self.weapons {
            collidables.push(weapon);
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

    fn draw_weapons(&mut self, window: &mut RenderWindow) {
        for weapon in &mut self.weapons {
            weapon.draw(window);
        }
    }
}

impl Updatable for Level {
    fn update(&mut self, delta_time: f32) {
        self.update_player(delta_time);
        self.update_weapon_spawners(delta_time);
        self.update_weapons(delta_time);
        self.update_wave_spawner(delta_time);
        self.update_enemies(delta_time);
        self.update_word(delta_time);
    }
}

impl Drawable for Level {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.draw_player(window);
        self.draw_enemies(window);
        self.draw_weapons(window);
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
        self.weapon_spawner.start_spawning();
    }
}
