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