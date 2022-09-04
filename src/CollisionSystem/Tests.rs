use super::*;

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

    assert!(Collider::collide(first_collider, second_collider).is_some());

    second_collider.position = Vector::new(-0.5, 0.0);
    assert!(Collider::collide(first_collider, second_collider).is_some());

    second_collider.position = Vector::new(-0.5, 0.5);
    assert!(Collider::collide(first_collider, second_collider).is_some());

    second_collider.position = Vector::new(-1.0, 0.0);
    assert!(Collider::collide(first_collider, second_collider).is_none());

    second_collider.position = Vector::new(-1.0, -1.0);
    assert!(Collider::collide(first_collider, second_collider).is_none());

    second_collider.position = Vector::new(-0.99, -0.99);
    assert!(Collider::collide(first_collider, second_collider).is_some());

    second_collider.position = Vector::new(0.99, 0.99);
    assert!(Collider::collide(first_collider, second_collider).is_some());
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
        shape: CollisionShape::Circle(Circle::new(size_for_both_colliders)),
    };

    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(-0.5, -0.5);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(0.5, 0.0);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(0.5, 1.0);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(0.5, 0.5);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(0.5, -0.99);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_some());

    circular_collider.position = Vector::new(0.5, -2.0);
    assert!(Collider::collide(rectangle_collider, circular_collider).is_none());

    //symetric calls
    circular_collider.position = Vector::new(-0.5, -0.5);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_some());

    circular_collider.position = Vector::new(0.5, 0.0);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_some());

    circular_collider.position = Vector::new(0.5, 1.0);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_some());

    circular_collider.position = Vector::new(0.5, 0.5);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_some());

    circular_collider.position = Vector::new(0.5, -0.99);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_some());

    circular_collider.position = Vector::new(0.5, -2.0);
    assert!(Collider::collide(circular_collider, rectangle_collider).is_none());
}

#[test]
fn circle_to_circle_collision_test() {
    let radius = 1.0f32;
    let first_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Circle(Circle::new(radius)),
    };

    let mut second_collider = Collider {
        position: Vector::default(),
        shape: CollisionShape::Circle(Circle::new(radius)),
    };

    assert!(Collider::collide(first_collider, second_collider).is_some());

    second_collider.position = Vector::new(2.0, 0.0);
    assert!(Collider::collide(first_collider, second_collider).is_none());

    second_collider.position = Vector::new(1.99, 0.0);
    assert!(Collider::collide(first_collider, second_collider).is_some());
}
