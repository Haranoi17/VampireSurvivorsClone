use crate::{Animations::AnimationPlayer, CollisionSystem::{Collidable, CollisionShape, Rectangle}, MathUtilities::{Vector, Point}};

use sfml::{
    graphics::{RenderTarget, RenderWindow, Transformable, RectangleShape, Shape},
    system::Vector2f,
    window::Key,
};

use super::Interfaces::{Drawable, Initializable, Updatable};
use crate::CollisionSystem::Collider;

pub struct Player {
    animation_player: AnimationPlayer,
    
    position: Point,
    move_direction: Vector,
    speed: f32,
    collision_shape: CollisionShape

}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            animation_player: AnimationPlayer::new(),
            move_direction: Vector::new(0.0, 0.0),
            speed: 500.0,
            collision_shape: CollisionShape::Rectangle(Rectangle::new(40.0, 40.0))
        }
    }

    fn create_visual_representation(&self) -> RectangleShape {
        let mut visual_representation = RectangleShape::new();
        visual_representation.set_texture(self.animation_player.get_current_animation_frame(), false);
        visual_representation.set_position(self.position);
        visual_representation.set_size(Vector2f::new(30.0, 30.0));
        visual_representation
    }

    fn handle_input(&mut self) {
        self.handle_movement();
    }

    fn handle_movement(&mut self) {
        self.move_direction = Vector::new(0.0, 0.0);

        if Key::W.is_pressed() {
            self.move_direction += Vector::new(0.0, -1.0);
        }

        if Key::S.is_pressed() {
            self.move_direction += Vector::new(0.0, 1.0);
        }

        if Key::A.is_pressed() {
            self.move_direction += Vector::new(-1.0, 0.0);
        }

        if Key::D.is_pressed() {
            self.move_direction += Vector::new(1.0, 0.0);
        }

        self.move_direction = self.move_direction.normal().unwrap_or_default();
    }

    fn update_position(&mut self, delta_time: f32) {
        self.position += self.move_direction * self.speed * delta_time;
    }
}

impl Updatable for Player {
    fn update(&mut self, delta_time: f32) {
        self.animation_player.update(delta_time);

        self.handle_input();
        self.update_position(delta_time);
    }
}

impl Drawable for Player {
    fn draw(&mut self, window: &mut RenderWindow) {
        let visual_representation = self.create_visual_representation();
        window.draw(&visual_representation);
    }
}

impl Initializable for Player {
    fn initialize(&mut self) {
        let players_animations_path = String::from("resources/Animations/player");
        self.animation_player.initialize(players_animations_path);
    }
}

impl Collidable for Player{
    fn get_collider(&self)-> Collider {
        Collider { shape: self.collision_shape, position: self.position }
    }
}