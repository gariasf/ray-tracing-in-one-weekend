use std::cell::RefCell;
use std::rc::Rc;

use hittables::HittableList;
use vec3::Point3;

use crate::camera::Camera;
use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::utils::random_float;
use crate::vec3::Vec3;

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

    let material_ground = Rc::new(RefCell::new(Lambertian { albedo: Color::new(0.5, 0.5, 0.5) }));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground, Point3::new(0.0, 0.0, 0.0), false)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(a as f64 + 0.9 * random_float(), 0.2, b as f64 + 0.9 * random_float());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<RefCell<dyn material::MaterialTrait>>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(random_float(), random_float(), random_float());
                    material = Rc::new(RefCell::new(Lambertian { albedo }));
                    let center_1 = center + Vec3::new(0.0, random_float() * 0.5, 0.0);
                    world.add(Box::new(Sphere::new(center, 0.2, material, center_1, true)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(random_float(), random_float(), random_float());
                    let fuzz = random_float() * 0.5;
                    material = Rc::new(RefCell::new(Metal { albedo, fuzz }));
                    world.add(Box::new(Sphere::new(center, 0.2, material, Point3::new(0.0, 0.0, 0.0), false)));
                } else {
                    // glass
                    material = Rc::new(RefCell::new(Dielectric { refraction_index: 1.5 }));
                    world.add(Box::new(Sphere::new(center, 0.2, material, Point3::new(0.0, 0.0, 0.0), false)));
                }
            }
        }
    }


    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400f64;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    const VFOV: f64 = 20.0;

    const DEFOCUS_ANGLE: f64 = 1.0;
    const FOCUS_DISTANCE: f64 = 10.0;

    let look_from: Point3 = Point3::new(13.0, 2.0, 3.0);
    let look_at: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    // Camera
    let mut camera = Camera::new(
        ASPECT_RATIO, IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
        look_from,
        look_at,
        vup,
        DEFOCUS_ANGLE,
        FOCUS_DISTANCE,
    );

    let _ = camera.render(&world);
}
