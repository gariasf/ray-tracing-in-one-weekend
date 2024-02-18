use std::cell::RefCell;
use std::rc::Rc;

use hittables::HittableList;
use vec3::Point3;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittables;
mod utils;
mod interval;
mod camera;
mod material;

fn main() {
    // World
    let mut world: HittableList = HittableList::new();


    let material_ground = Rc::new(RefCell::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0)}));
    let material_center = Rc::new(RefCell::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) }));
    let material_left = Rc::new(RefCell::new(Dielectric { refraction_index: 1.5 }));
    let material_right = Rc::new(RefCell::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0 }));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400f64;

    // Camera
    let mut camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, 100, 50);

    let _ = camera.render(&world);
}
