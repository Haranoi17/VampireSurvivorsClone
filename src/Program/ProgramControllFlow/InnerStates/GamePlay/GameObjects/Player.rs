use std::sync::mpsc::Receiver;

use crate::{CollisionSystem::{Collidable, CollisionShape, Circle, CollisionMask, CollisionInfo}, MathUtilities::{Vector, Point, Position, self}, InputSystem::{InputConsumer, Input, Keys}};
use crate::Objects::Animations::AnimationPlayer;

use sfml::{
    graphics::{RenderTarget, RenderWindow, Transformable, Shape, CircleShape, Color, Sprite, RectangleShape}, system::Vector2f
};

use crate::Objects::Interfaces::{Drawable, Initializable, Updatable};
use crate::CollisionSystem::Collider;

pub mod WeaponSpawners;

pub struct Player {
    animation_player: AnimationPlayer,
    
    position: Point,
    move_direction: Vector,
    speed: f32,
    collision_shape: CollisionShape,
    color: Color,
    face_direction: i8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            animation_player: AnimationPlayer::new(),
            move_direction: Vector::new(0.0, 0.0),
            speed: 200.0,
            collision_shape: CollisionShape::Circle(Circle::new(40.0)),
            color: Color::RED,
            face_direction: 1
        }
    }

    fn create_visual_representation(&self) -> RectangleShape {
        let size: Vector2f = match self.collision_shape {
            CollisionShape::Circle(circle) => {Vector2f::new(circle.radius*2.0, circle.radius*2.0)}
            CollisionShape::Rectangle(rectangle) =>{Vector2f::new(rectangle.width, rectangle.height)}
        };
        
        let texture = self.animation_player.get_current_animation_frame();
        
        let mut visual_representation = RectangleShape::with_size(size);
        visual_representation.set_texture(texture, false);
        visual_representation.set_position(self.position);
        visual_representation.set_origin(size/2.0);
        visual_representation.set_scale(Vector2f::new(1.5*self.face_direction as f32, 1.5));
        visual_representation
    }

    fn handle_movement(&mut self, input: &Input) {
        self.move_direction = Vector::new(0.0, 0.0);

        if input.is_pressed(Keys::Up) {
            self.move_direction += Vector::new(0.0, -1.0);
        }

        if input.is_pressed(Keys::Down) {
            self.move_direction += Vector::new(0.0, 1.0);
        }

        if input.is_pressed(Keys::Left) {
            self.move_direction += Vector::new(-1.0, 0.0);
        }

        if input.is_pressed(Keys::Right) {
            self.move_direction += Vector::new(1.0, 0.0);
        }

        self.move_direction = self.move_direction.normal().unwrap_or_default();
    }

    fn update_face_direction(&mut self)
    {
        if(self.move_direction.get_x() > 0.0)
        {
            self.face_direction = 1;
        }
        else if (self.move_direction.get_x() < 0.0) {
            self.face_direction = -1;
        }
    }

    fn update_position(&mut self, delta_time: f32) {
        self.position += self.move_direction * self.speed * delta_time;
    }

    pub fn on_collision(&mut self){
        self.color = Color::GREEN;
    }

    pub fn get_position(&self)->Position{
        self.position
    }

    fn prevent_walking_on_other_objects(&mut self, info: CollisionInfo){
        self.position = self.position - info.collision_depth;
    }
}

impl Updatable for Player {
    fn update(&mut self, delta_time: f32) {
        self.animation_player.update(delta_time);

        self.update_face_direction();
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
        Collider { shape: self.collision_shape, position: self.position,  }
    }

    fn get_mask(&self) -> CollisionMask {
        CollisionMask::Player
    }

    fn react_to_collision(&mut self, info: CollisionInfo, other_mask: CollisionMask) {
        match other_mask {
            CollisionMask::Player => {
                self.prevent_walking_on_other_objects(info);
            },
            CollisionMask::Enemy => {
                self.prevent_walking_on_other_objects(info);
            },
            CollisionMask::Weapon =>{/* do nothing */}
        }
    }
}

impl InputConsumer for Player{
    fn handle_input(&mut self, input: &crate::InputSystem::Input) {
        self.handle_movement(input);
    }
}