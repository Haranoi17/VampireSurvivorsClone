use std::any::Any;

use sfml::graphics::RenderWindow;

pub trait Updatable {
    fn update(&mut self, delta_time: f32);
}

pub trait Drawable {
    fn draw(&mut self, window: &mut RenderWindow);
}

pub trait Initializable {
    fn initialize(&mut self){}
}

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}