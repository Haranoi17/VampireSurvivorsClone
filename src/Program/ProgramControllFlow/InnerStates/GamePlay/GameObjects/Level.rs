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

use super::{Enemy::Enemy, Player::Player};

struct EnemySpawner {
    timer: BasicTimer,
}

impl EnemySpawner {
    pub fn new(spawn_interval: f32) -> Self {
        Self {
            timer: BasicTimer::new(spawn_interval),
        }
    }

    pub fn start_spawning(&mut self) {
        self.timer.start();
    }

    pub fn spawn(&mut self) -> Vec<Enemy> {
        vec![Enemy::new(Position::new(20.0, 20.0))]
    }

    fn should_spawn(&self) -> bool {
        self.timer.isFinished()
    }
}

impl Updatable for EnemySpawner {
    fn update(&mut self, delta_time: f32) {
        self.timer.update(delta_time);
    }
}

pub struct Level {
    player: Player,
    enemies: Vec<Enemy>,

    collision_symulation: WordSymulation,
    enemy_spawner: EnemySpawner,
}

impl Level {
    pub fn new() -> Self {
        let mut new_level = Self {
            player: Player::new(),
            enemies: vec![],
            collision_symulation: WordSymulation::new(),
            enemy_spawner: EnemySpawner {
                timer: BasicTimer::new(2.0),
            },
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
            self.enemies.extend( self.enemy_spawner.spawn() );
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
            .gather_collision_info(&collidables);
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
