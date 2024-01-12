use core::f64;
use rand::Rng;
use rand_distr::{num_traits::Float, StandardNormal};
use std::ops::{Add, Mul, Sub};

use crate::objloader::load_obj_file;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
    pub fn on_unit_sphere() -> Vector3 {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.sample(StandardNormal);
        let y: f64 = rng.sample(StandardNormal);
        let z: f64 = rng.sample(StandardNormal);
        Vector3::new(x, y, z).normalize()
    }
    pub fn as_col(&self) -> Col {
        // Apply gamma correction with sqrt
        fn col_cast(f: f64) -> u8 {
            (clamp(f, 0.0, 1.0).sqrt() * 255.0) as u8
        }
        Col {
            r: col_cast(self.x),
            g: col_cast(self.y),
            b: col_cast(self.z),
        }
    }
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vector3 {
        let l = self.norm();
        Vector3::new(self.x / l, self.y / l, self.z / l)
    }
    pub fn star(&self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    pub fn lerp(a: Vector3, b: Vector3, value: f64) -> Vector3 {
        let f = clamp(value, 0.0, 1.0);
        ((1.0 - f) * a + f * b).normalize()
    }
    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}
fn lerp(a: f64, b: f64, value: f64) -> f64 {
    let f = clamp(value, 0.0, 1.0);
    (1.0 - f) * a + f * b
}

// A normally-distributed random variable
// Global functions may be the answer, in this case.
fn rand_normal() -> f64 {
    rand::thread_rng().sample(StandardNormal)
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
// Dot product
impl Mul for Vector3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
// Implement the Mul trait for i32 and Vector3
impl Mul<Vector3> for f64 {
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

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Vector3,
    pub dir: Vector3,
    // Horizontal fov IN RADIANS
    pub fov: f64,
}

pub struct Mesh {
    pos: Vector3,
    scale: f64,
    tris: Vec<Triangle>,
}
impl Mesh {
    pub fn new(pos: Vector3, scale: f64, tris: Vec<Triangle>) -> Mesh {
        Mesh { pos, scale, tris }
    }
    pub fn from(path: String, pos: Vector3, scale: f64) -> Mesh {
        Mesh::new(pos, scale, load_obj_file(path))
    }
}
impl Geometry for Mesh {
    /// Ray-mesh intersection. Simply executes intersection for all triangles in mesh.
    fn intersect(&self, origin: Vector3, dir: Vector3) -> Option<(f64, Vector3)> {
        // Find best intersection
        let mut best = None;
        let mut best_depth = f64::MAX;
        for t in &self
            .tris
            .iter()
            .map(|t| {
                Triangle::new(
                    self.scale * t.a + self.pos,
                    self.scale * t.b + self.pos,
                    self.scale * t.c + self.pos,
                )
            })
            .collect::<Vec<Triangle>>()
        {
            let res = t.intersect(origin, dir);
            match res {
                Some((d, _)) if d < best_depth => {
                    best = res;
                    best_depth = d;
                }
                _ => (),
            }
        }
        best
    }
}

pub trait Geometry {
    /// add code here
    /// Defines the intersection behaviour for a ray.
    /// Returns: option, (intersection depth, normal at intersection)
    fn intersect(&self, origin: Vector3, dir: Vector3) -> Option<(f64, Vector3)>;
}

/*
pub struct Plane {
    position: Vector3,
    normal: Vector3,
}
impl Plane {
    pub fn new(position: Vector3, normal: Vector3) -> Self {
        Plane { position, normal }
    }
}
impl Geometry for Plane {
    /// Ray-plane intersection. Returns an option(depth, ws_normal).
    /// References a depth value, used for depth culling.
    /// Assumes dir is normalized
    /// returns [intersection found?, intersection depth, normal at point of intersection.]
    fn intersect(&self, origin: Vector3, dir: Vector3) -> Option<(f64, Vector3)> {
        let dotpr = dir * self.normal;
        if dotpr <= 0.0 {
            return None;
        }
        if (dotpr > -f64::EPSILON && dotpr < f64::EPSILON) {
            return None;
        }
        // TODO: Implement
        unimplemented!()
    }
}*/

#[derive(Debug)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}
impl Geometry for Sphere {
    /// Ray-sphere intersection. Returns a boolean.
    /// References a depth value, used for depth culling.
    /// Assumes dir is normalized
    /// returns [intersection found?, intersection depth, normal at point of intersection.]
    fn intersect(&self, origin: Vector3, dir: Vector3) -> Option<(f64, Vector3)> {
        let l = self.center - origin; // origin to sphere center
        let tca = l * dir; // dot of ray dir, and that of origin to circle center
                           //println!("{}", tca);
        let d2 = l * l - tca * tca; // square of distance from center of sphere to closest
                                    // approach
        let r2 = self.radius * self.radius; // radius squared
        if d2 > r2 {
            //println!("{}, {}, {}", d2 - r2, d2, r2);
            // distance grater than radius, no intersection
            return None;
        }
        let thc: f64 = (r2 - d2).sqrt(); // radius
        let z = tca - thc; // Difference between radius and closest ray approach
        let z1 = tca + thc; // The new depth value
        if z > 0.0 {
            let normal = (origin + (z * dir) - self.center).normalize();
            return Some((z, normal));
        }
        if z1 > 0.0 {
            let normal = (origin + (z1 * dir) - self.center).normalize();
            return Some((z1, normal));
        }
        /*if *z0 < 0.0 {
            // depth negative, cull (lies behind camera)
            return None;
        };*/
        // Our vectors are in world space
        return None;
    }
}

#[derive(Debug)]
pub struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}
impl Triangle {
    // Vertices
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
        Triangle { a, b, c }
    }
}
impl Geometry for Triangle {
    /// Ray-triangle intersection. Returns a boolean.
    /// From https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    /// References a depth value, used for depth culling.
    /// Assumes dir is normalized
    /// returns [intersection found?, intersection depth, normal at point of intersection.]
    fn intersect(&self, origin: Vector3, dir: Vector3) -> Option<(f64, Vector3)> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let n: Vector3 = ab.cross(ac).normalize();
        if -1.0 * n * dir < 0.0 {
            return None;
        }

        let d_cross_ac = dir.cross(ac);
        let det = ab * d_cross_ac;

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // Ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = origin - self.a;
        let u = inv_det * s * d_cross_ac;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_ab = s.cross(ab);
        let v = inv_det * (dir * s_cross_ab);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // depth of intersection (t is the parameter)
        let t = inv_det * (ac * s_cross_ab);

        if t > f64::EPSILON {
            Some((t, n))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub base_col: Vector3,
    pub emissive: Vector3,
    pub roughness: f64,
    pub metallic: f64,
    // Normals will be more convoluted
}
impl Material {
    fn new(base_col: Vector3, emissive: Vector3, roughness: f64, metallic: f64) -> Material {
        Material {
            base_col,
            emissive,
            roughness,
            metallic,
        }
    }

    pub fn gray_mat() -> Material {
        Material::new(
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(0.0, 0.0, 0.0),
            1.0,
            0.0,
        )
    }
    pub fn semirough() -> Material {
        Material::new(
            Vector3::new(0.9, 0.3, 0.3),
            Vector3::new(0.0, 0.0, 0.0),
            0.5,
            0.0,
        )
    }
    pub fn bluish() -> Material {
        Material::new(
            Vector3::new(0.2, 0.2, 0.5),
            Vector3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
        )
    }
    pub fn white_light() -> Material {
        Material::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(10.0, 10.0, 10.0),
            0.0,
            0.0,
        )
    }
    pub fn yellowish_light() -> Material {
        Material::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(12.0, 12.0, 8.0),
            0.0,
            0.0,
        )
    }
}

// Don't really know how I could do this...
//#[derive(Debug)]
pub struct Renderable {
    pub material: Material,
    pub geometry: Box<dyn Geometry>,
}
