use rand::{thread_rng, Rng};
use crate::{
    MathUtilities::Position,
    Objects::{
        Interfaces::{Updatable},
        Timers::{BasicTimer, Timer},
    },
    Program::ProgramControllFlow::InnerStates::GamePlay::GameObjects::{Level::Wave, Spawner::Spawner},
};

use super::Enemy;

pub struct WaveSpawner {
    timer: BasicTimer,
    wave_data: Wave,
}
impl WaveSpawner {
    pub fn new(wave: Wave) -> Self {
        Self {
            timer: BasicTimer::new(wave.enemy_spawn_delay_in_seconds),
            wave_data: wave,
        }
    }
}

impl Spawner<Enemy> for WaveSpawner {
    fn start_spawning(&mut self) {
        self.timer.start();
    }

    fn stop_spawning(&mut self) {
        self.timer.stop();
    }

    fn spawn(&mut self) -> Vec<Enemy> {
        self.decrease_enemy_count();
        vec![self.create_enemy_at_random_position_outside_window()]
    }

    fn should_spawn(&self) -> bool {
        self.timer.isFinished()
    }

    fn finished_spawning(&self) -> bool {
        self.wave_data.enemy_count == 0
    }
}

impl WaveSpawner {
    fn decrease_enemy_count(&mut self) {
        self.wave_data.enemy_count -= 1;
    }

    fn create_enemy_at_random_position_outside_window(&self)->Enemy{
        let mut rng = thread_rng(); 
        let mut side_rng = thread_rng();
        let side = side_rng.gen_range(0..4);
        
        //sooooo lazyyyyyy....
        let mut random_pos = Position::default();
        if side == 0{
            random_pos = Position::new(rng.gen_range(-20..-10) as f32, rng.gen_range(-10..800) as f32);
        }else if side == 1{
            random_pos = Position::new(rng.gen_range(-10..1300) as f32, rng.gen_range(-20..-10) as f32);
        }else if side == 2{
            random_pos = Position::new(rng.gen_range(1300..1320) as f32, rng.gen_range(-10..800) as f32);
        }else if side == 3{
            random_pos = Position::new(rng.gen_range(-10..1300) as f32, rng.gen_range(800..810) as f32);
        }


        Enemy::new(random_pos)
    }
}

impl Updatable for WaveSpawner {
    fn update(&mut self, delta_time: f32) {
        self.timer.update(delta_time);
    }
}
