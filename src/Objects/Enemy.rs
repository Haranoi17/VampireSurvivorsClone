use sfml::graphics::{ RectangleShape, RenderTarget, Transformable};
use sfml::system::Vector2f;

use super::Interfaces::{Drawable, Initializable, Updatable};
use crate::CollisionSystem::{Collidable, Collider, CollisionShape, Rectangle};
use crate::MathUtilities::Point;

pub struct Enemy {
    position: Point,
    rectangle: Rectangle,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            position: Point::new(200.0, 200.0),
            rectangle: Rectangle::new(40.0, 40.0),
        }
    }
}

impl Updatable for Enemy {
    fn update(&mut self, delta_time: f32) {}
}

impl Drawable for Enemy {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        let mut rect = RectangleShape::new();
        rect.set_size(Vector2f::new(self.rectangle.width, self.rectangle.height));
        rect.set_position(self.position);
        window.draw(&rect);
    }
}

impl Collidable for Enemy {
    fn get_collider(&self) -> Collider {
        Collider {
            shape: CollisionShape::Rectangle(self.rectangle),
            position: self.position,
        }
    }
}
