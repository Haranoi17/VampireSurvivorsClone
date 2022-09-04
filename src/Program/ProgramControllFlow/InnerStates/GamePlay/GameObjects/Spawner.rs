use crate::Objects::Interfaces::{Updatable, Drawable};

pub trait Spawner<T> {
    fn start_spawning(&mut self);
    fn stop_spawning(&mut self);
    fn spawn(&mut self) -> Vec<T>;
    fn should_spawn(&self) -> bool;
    fn finished_spawning(&self) -> bool;
}