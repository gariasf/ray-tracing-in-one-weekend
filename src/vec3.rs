use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::utils::{random_float, random_float_range};

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.elements[0] * self.elements[0] + self.elements[1] * self.elements[1] + self.elements[2] * self.elements[2]
    }

    pub(crate) fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s: f64 = 1e-8;
        self.elements[0].abs() < s && self.elements[1].abs() < s && self.elements[2].abs() < s
    }


}

pub(crate) fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

pub(crate) fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = dot(-uv, n).min(1.0);
    let r_out_perp: Vec3 = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel: Vec3 = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
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

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.elements[0] * other.elements[0], self.elements[1] * other.elements[1], self.elements[2] * other.elements[2])
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

pub(crate) fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.elements[1] * v.elements[2] - u.elements[2] * v.elements[1],
        u.elements[2] * v.elements[0] - u.elements[0] * v.elements[2],
        u.elements[0] * v.elements[1] - u.elements[1] * v.elements[0],
    )
}

pub(crate) fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub(crate) fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            continue;
        }
        return p;
    }
}

pub(crate) fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub(crate) fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_vector();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub(crate) fn random() -> Vec3 {
    Vec3::new(random_float(), random_float(), random_float())
}

pub(crate) fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_float_range(min, max),
        random_float_range(min, max),
        random_float_range(min, max),
    )
}

// Alias for geometric clarity
pub(crate) type Point3 = Vec3;

