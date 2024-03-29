pub mod Symulation;
mod Tests;

use std::vec;

use crate::MathUtilities::{Point, Position, Vector};

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius: radius }
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy)]
pub enum CollisionShape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Clone, Copy)]
pub struct CollisionInfo {
    pub collision_depth: Vector,
}

impl CollisionInfo {
    pub fn new(collision_depth: Vector) -> Self {
        Self { collision_depth }
    }

    pub fn symetrical(&self) -> Self {
        Self {
            collision_depth: -self.collision_depth,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Collider {
    pub shape: CollisionShape,
    pub position: Position,
}

#[derive(Clone, Copy)]
pub enum CollisionMask {
    Player,
    Weapon,
    Enemy,
}

pub trait Collidable {
    fn get_collider(&self) -> Collider;
    fn get_mask(&self) -> CollisionMask;
    fn react_to_collision(&mut self, info: CollisionInfo, other_mask: CollisionMask);
}

impl Collider {
    pub fn collide(first: Self, second: Self) -> Option<CollisionInfo> {
        match (first.shape, second.shape) {
            (CollisionShape::Circle(first_circle), CollisionShape::Circle(second_circle)) => {
                Self::circle_to_circle_collision(
                    first.position,
                    first_circle.radius,
                    second.position,
                    second_circle.radius,
                )
            }
            (CollisionShape::Circle(circle), CollisionShape::Rectangle(rectangle)) => {
                Self::circle_to_rectangle_collision(
                    first.position,
                    circle.radius,
                    second.position,
                    rectangle,
                )
            }
            (CollisionShape::Rectangle(rectangle), CollisionShape::Circle(circle)) => {
                Self::circle_to_rectangle_collision(
                    second.position,
                    circle.radius,
                    first.position,
                    rectangle,
                )
            }
            (
                CollisionShape::Rectangle(first_rectangle),
                CollisionShape::Rectangle(second_rectangle),
            ) => Self::rectangle_to_rectangle_collision(
                first.position,
                first_rectangle,
                second.position,
                second_rectangle,
            ),
        }
    }

    fn circle_to_circle_collision(
        first_position: Position,
        first_radius: f32,
        second_position: Position,
        second_radius: f32,
    ) -> Option<CollisionInfo> {
        let distance_between_circles = Point::distance(first_position, second_position);
        let sum_of_radii = first_radius + second_radius;

        let are_colliding = distance_between_circles < sum_of_radii;

        if !are_colliding {
            return None;
        }

        let direction = (second_position - first_position)
            .normal()
            .unwrap_or_default();
        let depth = (second_position - first_position).length();

        Some(CollisionInfo::new(direction * depth))
    }

    fn circle_to_rectangle_collision(
        circle_position: Position,
        circle_radius: f32,
        rectangle_position: Position,
        rectangle: Rectangle,
    ) -> Option<CollisionInfo> {
        let rectangle_offset_position =
            rectangle_position - Vector::new(rectangle.width, rectangle.height) * 0.5;

        let nearest_x = f32::max(
            rectangle_offset_position.get_x(),
            f32::min(
                circle_position.get_x(),
                rectangle_offset_position.get_x() + rectangle.width,
            ),
        );
        let nearest_y = f32::max(
            rectangle_offset_position.get_y(),
            f32::min(
                circle_position.get_y(),
                rectangle_offset_position.get_y() + rectangle.height,
            ),
        );

        let nearest_on_rectangle = Vector::new(nearest_x, nearest_y);

        let difference = nearest_on_rectangle - circle_position;

        let are_colliding = difference.length() < circle_radius;

        if !are_colliding{
            return None
        }

        let direction = (nearest_on_rectangle - circle_position).normal().unwrap_or_default();
        let point_on_circle = circle_position + direction * circle_radius;

        Some(CollisionInfo::new((nearest_on_rectangle - point_on_circle)))
    }

    fn rectangle_to_rectangle_collision(
        first_position: Position,
        first: Rectangle,
        second_position: Position,
        second: Rectangle,
    ) -> Option<CollisionInfo> {
        let first_offset_position = first_position - Vector::new(first.width, first.height) * 0.5;
        let second_offset_position =
            second_position - Vector::new(second.width, second.height) * 0.5;

        let are_colliding = first_offset_position.get_x() < second_offset_position.get_x() + second.width
            && first_offset_position.get_x() + first.width > second_offset_position.get_x()
            && first_offset_position.get_y() < second_offset_position.get_y() + second.height
            && first_offset_position.get_y() + first.height > second_offset_position.get_y();

        if !are_colliding{
            return None
        }
         
        let vector_between_centers = second_offset_position - first_offset_position;
        let normal = vector_between_centers.normal().unwrap_or_default();

        let first_support_point = first_position + Vector::new(normal.get_x().signum() * (first.width / 2.0), normal.get_y().signum() * (first.height/2.0));
        let second_support_point = second_position + Vector::new(-normal.get_x().signum() * (second.width / 2.0), -normal.get_y().signum() * (second.height/2.0));

        let mut depth_vector = first_support_point - second_support_point;
        
        if depth_vector.get_x().abs() > depth_vector.get_y().abs(){
            depth_vector = Vector::new(0.0, depth_vector.get_y());
        }
        else{
            depth_vector = Vector::new(depth_vector.get_x(), 0.0);
        }

        let depth = depth_vector * 0.5;

        Some(CollisionInfo::new(depth))
    }

    fn is_point_in_rectangle(
        rectangle_position: Position,
        rectangle: Rectangle,
        point: Point,
    ) -> bool {
        let rectangle_offset_position =
            rectangle_position - Vector::new(rectangle.width, rectangle.height) * 0.5;

        let is_in_x = point.get_x() > rectangle_offset_position.get_x()
            && point.get_x() < rectangle_offset_position.get_x() + rectangle.width;
        let is_in_y = point.get_y() > rectangle_offset_position.get_y()
            && point.get_y() < rectangle_offset_position.get_y() + rectangle.height;

        let result = is_in_x && is_in_y;

        result
    }

    pub fn outer_rectangle_circle(rectangle: Rectangle) -> Circle {
        let radius =
            f32::sqrt(f32::powi(rectangle.height, 2) + f32::powi(rectangle.width, 2)) / 2.0;
        Circle::new(radius)
    }
}
