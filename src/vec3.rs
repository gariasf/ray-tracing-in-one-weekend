use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Vec3 {
    elements: [f64; 3],
}

impl Vec3 {
    pub(crate) fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { elements: [e0, e1, e2] }
    }

    pub(crate) fn x(&self) -> f64 {
        self.elements[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self.elements[1]
    }

    pub(crate) fn z(&self) -> f64 {
        self.elements[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.elements[0] * self.elements[0] + self.elements[1] * self.elements[1] + self.elements[2] * self.elements[2]
    }


}

// Negation
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.elements[0], -self.elements[1], -self.elements[2])
    }
}

// Indexing
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.elements[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elements[i]
    }
}

// Addition
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.elements[0] + other.elements[0], self.elements[1] + other.elements[1], self.elements[2] + other.elements[2])
    }
}

// Subtraction
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.elements[0] - other.elements[0], self.elements[1] - other.elements[1], self.elements[2] - other.elements[2])
    }
}

// Scalar Multiplication
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.elements[0] * t, self.elements[1] * t, self.elements[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// Division by scalar
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

// Display
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.elements[0], self.elements[1], self.elements[2])
    }
}

// Utility Functions
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.elements[0] * v.elements[0] + u.elements[1] * v.elements[1] + u.elements[2] * v.elements[2]
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.elements[1] * v.elements[2] - u.elements[2] * v.elements[1],
        u.elements[2] * v.elements[0] - u.elements[0] * v.elements[2],
        u.elements[0] * v.elements[1] - u.elements[1] * v.elements[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// Alias for geometric clarity
pub(crate) type Point3 = Vec3;

