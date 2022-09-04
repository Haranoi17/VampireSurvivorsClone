use crate::{
    MathUtilities::Position,
    Program::ProgramControllFlow::InnerStates::GamePlay::GameObjects::{
        Spawner::Spawner, Weapons::SimpleMissile::SimpleMissile,
    }, Objects::{Timers::{BasicTimer, Timer}, Interfaces::Updatable},
};

pub struct SimpleMissileSpawner {
    timer: BasicTimer,
    spawn_position: Position,
    target_position: Position,
}

impl SimpleMissileSpawner {
    pub fn new(spawn_delay: f32)->Self{
        Self { timer: BasicTimer::new(spawn_delay), spawn_position: Position::default(), target_position: Position::default()  }
    }

    pub fn set_target_position(&mut self, target_position: Position){
        self.target_position = target_position;
    }

    pub fn set_spawn_position(&mut self, spawn_position: Position){
        self.spawn_position = spawn_position;
    }
}

impl Spawner<SimpleMissile> for SimpleMissileSpawner {
    fn finished_spawning(&self) -> bool {
        false
    }

    fn should_spawn(&self) -> bool {
        self.timer.isFinished()
    }

    fn spawn(&mut self) -> Vec<SimpleMissile> {
        let new_simple_missile = SimpleMissile::new(self.spawn_position, self.target_position);
        vec![new_simple_missile]
    }

    fn start_spawning(&mut self) {
        self.timer.start();
    }

    fn stop_spawning(&mut self) {
        self.timer.stop();
    }
}

impl Updatable for SimpleMissileSpawner {
    fn update(&mut self, delta_time: f32) {
        self.timer.update(delta_time);
    }
}