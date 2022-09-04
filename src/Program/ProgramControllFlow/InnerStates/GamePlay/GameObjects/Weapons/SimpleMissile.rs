use sfml::graphics::{Color, CircleShape, Transformable, Shape, RenderTarget};
use crate::CollisionSystem::{Collider, CollisionMask, CollisionInfo};

use crate::{
    CollisionSystem::{Circle, CollisionShape, Collidable},
    MathUtilities::{Position, Vector},
    Objects::Interfaces::{Updatable, Drawable},
};

pub struct SimpleMissile {
    start_position: Position,
    position: Position,
    damage: usize,
    speed: f32,
    shape: Circle,
    fly_direction: Vector,
    fly_distance: f32,
    should_be_destroyed: bool
}

impl SimpleMissile {
    pub fn new(start_position: Position, target_position: Position) -> Self {
        Self {
            start_position: start_position,
            position:(start_position),
            damage: 10,
            speed: 200.0,
            fly_direction: (target_position - start_position).normal().unwrap(),
            fly_distance: 500.0,
            shape: Circle { radius: 5.0 },
            should_be_destroyed: false
        }
    }

    pub fn should_be_destroyed(&self)->bool{
        self.should_be_destroyed
    }
}

impl Updatable for SimpleMissile {
    fn update(&mut self, delta_time: f32) {
        if (self.start_position - self.position).length() > self.fly_distance{
            self.should_be_destroyed = true;
        }

        self.position = self.position + self.fly_direction * delta_time * self.speed;
    }
}

impl Drawable for SimpleMissile{
    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        let mut visual_representation = CircleShape::new(self.shape.radius, 100);
        visual_representation.set_position(self.position);
        visual_representation.set_origin(Vector::new(self.shape.radius, self.shape.radius));
        visual_representation.set_fill_color(Color::RED);

        window.draw(&visual_representation);
    }
}

impl Collidable for SimpleMissile{
    fn get_collider(&self) -> Collider {
        Collider {
            shape: CollisionShape::Circle(self.shape),
            position: self.position,
        }
    }

    fn get_mask(&self) -> CollisionMask {
        CollisionMask::Weapon
    }

    fn react_to_collision(&mut self, info: CollisionInfo, other_mask: CollisionMask) {
        match other_mask{
            CollisionMask::Player => {/* do nothing */},
            CollisionMask::Weapon => {/* do nothing */},
            CollisionMask::Enemy => {self.should_be_destroyed = true},
        }       
    }
}