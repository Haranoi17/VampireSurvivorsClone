use crate::MathUtilities::{Point, Position};

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
pub struct Collider {
    pub shape: CollisionShape,
    pub position: Position,
}

pub trait Collidable {
    fn get_collider(&self) -> Collider;
}

impl Collider {
    pub fn collide(first: Self, second: Self) -> bool {
        match (first.shape, second.shape) {
            (CollisionShape::Circle(first_circle), CollisionShape::Circle(second_circle)) => {
                Self::circle_to_circle_collision(
                    first.position,
                    first_circle.radius,
                    second.position,
                    second_circle.radius,
                )
            }
            (CollisionShape::Circle(circle), CollisionShape::Rectangle(rectangle))
            | (CollisionShape::Rectangle(rectangle), CollisionShape::Circle(circle)) => {
                Self::circle_to_rectangle_collision(
                    first.position,
                    circle.radius,
                    second.position,
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
            )
        }
    }

    fn circle_to_circle_collision(
        first_position: Position,
        first_radius: f32,
        second_position: Position,
        second_radius: f32,
    ) -> bool {
        Point::distance(first_position, second_position) < first_radius + second_radius
    }

    fn circle_to_rectangle_collision(
        circle_position: Position,
        circle_radius: f32,
        rectangle_position: Position,
        rectangle: Rectangle,
    ) -> bool {
        let circle_collider = Collider {
            shape: CollisionShape::Circle(Circle::new(circle_radius)),
            position: circle_position,
        };

        let outer_rectangle_circle_collider = Collider {
            shape: CollisionShape::Circle(Self::outer_rectangle_circle(rectangle)),
            position: rectangle_position,
        };

        if !Collider::collide(circle_collider, outer_rectangle_circle_collider) {
            return false;
        }

        // let circle_center = circle
        // let point_on_circle_between_colliders_centers =
        true
    }

    fn rectangle_to_rectangle_collision(
        first_position: Position,
        first: Rectangle,
        second_position: Position,
        second: Rectangle,
    ) -> bool {
        let result = first_position.get_x() < second_position.get_x() + second.width
            && first_position.get_x() + first.width > second_position.get_x()
            && first_position.get_y() < second_position.get_y() + second.height
            && first_position.get_y() + first.height > second_position.get_y();

        result
    }

    // fn is_point_in_rectangle(rectangle: Rectangle, point: Point) {}

    fn outer_rectangle_circle(rectangle: Rectangle) -> Circle {
        Circle::new(f32::sqrt(
            f32::powi(rectangle.height, 2) + f32::powi(rectangle.width, 2),
        ))
    }
}
