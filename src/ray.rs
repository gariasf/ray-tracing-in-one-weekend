use crate::vec3::{Vec3};

#[derive(PartialEq)]
pub(crate) struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Ray { origin, direction, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}