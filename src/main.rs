mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittables;
mod utils;
mod interval;
mod camera;

use std::io::{Write};
use vec3::{Point3};
use hittable::{Hittable};
use hittables::{HittableList};
use crate::camera::Camera;

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(sphere::Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400f64;

    // Camera
    let mut camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100);

    let _ = camera.render(&world);
}
