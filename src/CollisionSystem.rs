use sfml::graphics::{RenderTarget, CircleShape, Transformable, Shape, Color, RectangleShape};

use crate::{MathUtilities::{Point, Position, Vector}, Objects::Interfaces::Drawable};

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

// impl Drawable for Collider{
//     fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
//         match self.shape {
//             CollisionShape::Circle(circle) => {
//                 let mut circleShape = CircleShape::new(circle.radius, 100);
//                 circleShape.set_fill_color(Color::TRANSPARENT);
//                 circleShape.set_outline_color(Color::GREEN);
//                 circleShape.set_position(self.position);

//                 window.draw(&circleShape);
//             },
//             CollisionShape::Rectangle(rectangle) => {
//                 let mut rectangleShape = RectangleShape::new();
//                 rectangleShape.set_size(Vector::new(rectangle.width, rectangle.height));
//                 rectangleShape.set_fill_color(Color::TRANSPARENT);
//                 rectangleShape.set_outline_color(Color::GREEN);
//                 rectangleShape.set_position(self.position);

//                 window.draw(&rectangleShape);
//             },
//         }
//     }
// }

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
    ) -> bool {
        Point::distance(first_position, second_position) < first_radius + second_radius
    }

    fn circle_to_rectangle_collision(
        circle_position: Position,
        circle_radius: f32,
        rectangle_position: Position,
        rectangle: Rectangle,
    ) -> bool {
        let nearest_x = f32::max(rectangle_position.get_x(), f32::min(circle_position.get_x(), rectangle_position.get_x()+rectangle.width));
        let nearest_y = f32::max(rectangle_position.get_y(), f32::min(circle_position.get_y(), rectangle_position.get_y()+rectangle.height));

        let nearest_on_rectangle = Vector::new(nearest_x, nearest_y);

        let difference = nearest_on_rectangle - circle_position;

        let result = difference.length() < circle_radius;

        return result;
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

    fn is_point_in_rectangle(
        rectangle_position: Position,
        rectangle: Rectangle,
        point: Point,
    ) -> bool {
        let is_in_x = point.get_x() > rectangle_position.get_x()
            && point.get_x() < rectangle_position.get_x() + rectangle.width;
        let is_in_y = point.get_y() > rectangle_position.get_y()
            && point.get_y() < rectangle_position.get_y() + rectangle.height;

        let result = is_in_x && is_in_y;

        result
    }

    fn outer_rectangle_circle(rectangle: Rectangle) -> Circle {
        let radius = f32::sqrt(f32::powi(rectangle.height, 2) + f32::powi(rectangle.width, 2))/2.0;
        Circle::new(radius)
    }
}

#[test]
fn rectangle_rectangle_collision_tests() {
    let size_for_both_colliders = 1.0f32;
    
    let first_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Rectangle(Rectangle::new(
            size_for_both_colliders,
            size_for_both_colliders,
        )),
    };
    
    let mut second_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Rectangle(Rectangle::new(
            size_for_both_colliders,
            size_for_both_colliders,
        )),
    };

    assert_eq!(Collider::collide(first_collider, second_collider), true);

    second_collider.position = Vector::new(-0.5, 0.0);
    assert_eq!(Collider::collide(first_collider, second_collider), true);

    second_collider.position = Vector::new(-0.5, 0.5);
    assert_eq!(Collider::collide(first_collider, second_collider), true);
    
    second_collider.position = Vector::new(-1.0, 0.0);
    assert_eq!(Collider::collide(first_collider, second_collider), false);

    second_collider.position = Vector::new(-1.0, -1.0);
    assert_eq!(Collider::collide(first_collider, second_collider), false);

    second_collider.position = Vector::new(-0.99, -0.99);
    assert_eq!(Collider::collide(first_collider, second_collider), true);

    second_collider.position = Vector::new(0.99, 0.99);
    assert_eq!(Collider::collide(first_collider, second_collider), true);
}


#[test]
fn circle_rectangle_collision_tests() {
    let size_for_both_colliders = 1.0f32;
    
    let rectangle_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Rectangle(Rectangle::new(
            size_for_both_colliders,
            size_for_both_colliders,
        )),
    };
    
    let mut circular_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Circle(Circle::new(
            size_for_both_colliders,
        )),
    };

    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);

    circular_collider.position = Vector::new(-0.5, -0.5);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);

    circular_collider.position = Vector::new(0.5, 0.0);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);
    
    circular_collider.position = Vector::new(0.5, 1.0);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);

    circular_collider.position = Vector::new(0.5, 0.5);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);

    circular_collider.position = Vector::new(0.5, -0.99);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), true);
    
    circular_collider.position = Vector::new(0.5, -2.0);
    assert_eq!(Collider::collide(rectangle_collider, circular_collider), false);

    //symetric calls
    circular_collider.position = Vector::new(-0.5, -0.5);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), true);

    circular_collider.position = Vector::new(0.5, 0.0);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), true);
    
    circular_collider.position = Vector::new(0.5, 1.0);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), true);

    circular_collider.position = Vector::new(0.5, 0.5);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), true);

    circular_collider.position = Vector::new(0.5, -0.99);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), true);
    
    circular_collider.position = Vector::new(0.5, -2.0);
    assert_eq!(Collider::collide(circular_collider, rectangle_collider), false);

}

#[test]
fn circle_to_circle_collision_test(){
    let radius = 1.0f32;
    let first_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Circle(Circle::new(
            radius,
        )),
    };

    let mut second_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Circle(Circle::new(
            radius,
        )),
    };

    assert_eq!(Collider::collide(first_collider, second_collider), true);

    second_collider.position = Vector::new(2.0, 0.0);
    assert_eq!(Collider::collide(first_collider, second_collider), false);

    second_collider.position = Vector::new(1.99, 0.0);
    assert_eq!(Collider::collide(first_collider, second_collider), true);
}