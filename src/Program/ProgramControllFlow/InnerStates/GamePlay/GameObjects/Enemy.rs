use sfml::graphics::{ RectangleShape, RenderTarget, Transformable};
use sfml::system::Vector2f;

use crate::Objects::Interfaces::{Drawable, Initializable, Updatable};
use crate::CollisionSystem::{Collidable, Collider, CollisionShape, Rectangle, CollisionMask, CollisionInfo};
use crate::MathUtilities::{Position, Vector};

pub struct Enemy {
    position: Position,
    rectangle: Rectangle,
    speed: f32,
}

impl Enemy {
    pub fn new(position: Position) -> Self {
        Self {
            position: position,
            rectangle: Rectangle::new(40.0, 40.0),
            speed: 20.0,
        }
    }

    pub fn walk_towards(&mut self, target_position: Position, delta_time: f32){
        let option_direction = (target_position - self.position).normal();
        let direction = match option_direction {
            Some(direction) => self.position += direction * self.speed * delta_time,
            None => {},
        };
    }

    fn prevent_walking_on_other_objects(&mut self, info: CollisionInfo){
        self.position = self.position - info.collision_depth;
    }
}

impl Updatable for Enemy {
    fn update(&mut self, delta_time: f32) {}
}

impl Drawable for Enemy {
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        let mut rect = RectangleShape::new();
        let size = Vector::new(self.rectangle.width, self.rectangle.height);
        rect.set_size(size);
        rect.set_position(self.position);
        rect.set_origin(size*0.5);
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

    fn get_mask(&self) -> CollisionMask {
        CollisionMask::Enemy
    }

    fn react_to_collision(&mut self, info: CollisionInfo, other_mask: CollisionMask) {
        match other_mask {
            CollisionMask::Player => {
                self.prevent_walking_on_other_objects(info);
            },
            CollisionMask::Enemy => {
                self.prevent_walking_on_other_objects(info);
                
            },
        }
    }
}
