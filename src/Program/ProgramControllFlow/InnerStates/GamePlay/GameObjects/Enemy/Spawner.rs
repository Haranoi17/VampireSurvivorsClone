use crate::{Objects::{Timers::{BasicTimer, Timer}, Interfaces::Updatable}, MathUtilities::Position};

use super::Enemy;

pub struct Spawner {
    timer: BasicTimer,
}

impl Spawner {
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

    pub fn should_spawn(&self) -> bool {
        self.timer.isFinished()
    }
}

impl Updatable for Spawner {
    fn update(&mut self, delta_time: f32) {
        self.timer.update(delta_time);
    }
}
