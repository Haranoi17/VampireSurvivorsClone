use super::Interfaces::Updatable;

pub trait Timer: Updatable{
    fn start(&mut self);
    fn stop(&mut self);
    fn isFinished(&self)->bool;
    fn isActive(&self)->bool;
}

pub struct BasicTimer{
    duration: f32,
    remaining_time: f32,
    active: bool,
}

impl BasicTimer{
    pub fn new(duration: f32)->Self{
        Self { duration: duration, remaining_time: duration, active: false }
    }
}

impl BasicTimer{
    fn reset(&mut self){
        self.remaining_time = self.duration;
    }
}

impl Timer for BasicTimer{
    fn isFinished(&self)->bool {
        self.remaining_time <= 0.0
    }
    
    fn isActive(&self)->bool{
        self.active
    }

    fn start(&mut self) {
        self.active = true;
        self.reset();
    }

    fn stop(&mut self) {
        self.active = false;
        self.reset();
    }
}

impl Updatable for BasicTimer{
    fn update(&mut self, delta_time: f32) {
        if self.isFinished() || !self.isActive() || self.duration == 0.0{
            return
        }

        self.remaining_time -= delta_time;
    }
}

