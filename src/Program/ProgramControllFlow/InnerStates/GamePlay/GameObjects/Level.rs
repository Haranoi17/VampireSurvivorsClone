use std::{vec};

use sfml::graphics::RenderWindow;

use crate::{Objects::Interfaces::{Drawable, Initializable, Updatable}, InputSystem::{InputConsumer, Input}, CollisionSystem::{Collider, Symulation::WordSymulation, Collidable}};

use super::{Player::Player, Enemy::Enemy};

pub struct Level
{
    player: Player,
    enemies: Vec<Enemy>,

    collision_symulation: WordSymulation,
}

impl Level{
    pub fn new()->Self{
        let mut new_level = Self { player: Player::new(), enemies: vec![], collision_symulation: WordSymulation::new() };
        new_level.initialize();
        new_level
    }

    fn spawn_enemies(&mut self){
        for _ in 0..1{
            self.enemies.push(Enemy::new());
        }
    }

    fn update_player(&mut self, delta_time: f32){
        self.player.update(delta_time);
    }

    fn update_enemies(&mut self, delta_time: f32){
        for enemy in &mut self.enemies{
            enemy.update(delta_time);
            enemy.walk_towards(self.player.get_position(), delta_time);
        }  
    }

    fn update_word(&mut self, delta_time: f32){
        let mut collidables: Vec<&mut dyn Collidable> = vec![];
        for enemy in &mut self.enemies{
            collidables.push(enemy);
        }

        collidables.push(&mut self.player);

        self.collision_symulation.collision_detection(&collidables);
        self.collision_symulation.gather_collision_info(&collidables);
        self.collision_symulation.react_to_collisionss(&mut collidables);
        self.collision_symulation.clear_collisions();
        collidables.clear();
    }

    fn draw_player(&mut self, window: &mut RenderWindow){
        self.player.draw(window);
    }

    fn draw_enemies(&mut self, window: &mut RenderWindow){
        for enemy in &mut self.enemies{
            enemy.draw(window);
        }  
    }
}

impl Updatable for Level{
    fn update(&mut self, delta_time: f32) {
        self.update_player(delta_time);
        self.update_enemies(delta_time);
        self.update_word(delta_time);
    }
}

impl Drawable for Level{
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.draw_player(window);
        self.draw_enemies(window);
    }
}


impl InputConsumer for Level{
    fn handle_input(&mut self, input: &Input) {
        self.player.handle_input(input);
    }
}

impl Initializable for Level{
    fn initialize(&mut self) {
        self.spawn_enemies();
        self.player.initialize();
    }
}