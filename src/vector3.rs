use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Vector3 {
    pub fn new(xi: i32, yi: i32, zi: i32) -> Self {
        Vector3 {
            x: xi,
            y: yi,
            z: zi,
        }
    }
}
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
// Implement the Mul trait for i32 and Vector3
impl Mul<Vector3> for i32 {
    type Output = Vector3;

    // Scalar multiplication: Scalar * Vector3
    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
struct Sphere {
    center: Vector3,
    radius: i32,
}
impl Sphere {
    fn intersect(origin: Vector3, dir: Vector3) {
        ()
    }
}
