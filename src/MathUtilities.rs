mod Tests;

use sfml::system::Vector2f;
use std::ops::{AddAssign, Mul, Add, Sub};
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
        let length = self.length();

        if length <= 0.0 {
            return None;
        }

        Some(Vector::new(self.vector.x / length, self.vector.y / length))
    }

    pub fn length(&self)->f32{
        let length = f32::sqrt(f32::powi(self.vector.x, 2) + f32::powi(self.vector.y, 2));

        length
    }

    pub fn get_x(&self) -> f32 {
        self.vector.x
    }

    pub fn get_y(&self) -> f32 {
        self.vector.y
    }

    pub fn distance(end_point: Point, start_point: Point) -> f32 {
        let distance_vector = end_point.vector - start_point.vector;
        let length = f32::sqrt(f32::powi(distance_vector.x, 2) + f32::powi(distance_vector.y, 2));
    
        length
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

impl Add<Vector> for Vector{
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.get_x() + rhs.get_x(), self.get_y() + rhs.get_y())
    }

    type Output = Self;
}

impl Sub for Vector{
    fn sub(self, rhs: Self) -> Self::Output {
        let resutl_vector = self.vector - rhs.vector;
        Vector::new(resutl_vector.x, resutl_vector.y)
    }

    type Output = Vector;
}


impl Into<Vector2f> for Vector{
    fn into(self) -> Vector2f {
        self.vector
    }
}


pub type Position = Vector;
pub type Point = Vector;