use sfml::system::Vector2f;
use std::ops::{AddAssign, Mul};
use std::convert::Into;

#[derive(Clone, Copy, Default)]
pub struct Vector {
    vector: Vector2f,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2f::new(x, y),
        }
    }

    pub fn normal(&self) -> Option<Self> {
        let length = f32::sqrt(f32::powi(self.vector.x, 2) + f32::powi(self.vector.y, 2));

        if length <= 0.0 {
            return None;
        }

        Some(Vector::new(self.vector.x / length, self.vector.y / length))
    }

    pub fn get_x(&self) -> f32 {
        self.vector.x
    }

    pub fn get_y(&self) -> f32 {
        self.vector.y
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.vector += rhs.vector
    }
}

impl Mul<f32> for Vector {
    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.get_x() * rhs, self.get_y() * rhs)
    }

    type Output = Self;
}

#[derive(Clone, Copy, Default)]
pub struct Point {
    position: Vector2f,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2f::new(x, y),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.position.x
    }

    pub fn get_y(&self) -> f32 {
        self.position.y
    }
    
    pub fn distance(end_point: Point, start_point: Point) -> f32 {
        let distance_vector = end_point.position - start_point.position;
        let length = f32::sqrt(f32::powi(distance_vector.x, 2) + f32::powi(distance_vector.y, 2));
    
        length
    }
}

impl AddAssign<Vector> for Point{
    fn add_assign(&mut self, rhs: Vector) {
        self.position.x += rhs.get_x();
        self.position.y += rhs.get_y();
    }
}

impl Into<Vector2f> for Point{
    fn into(self) -> Vector2f {
        self.position
    }
}

pub type Position = Point;