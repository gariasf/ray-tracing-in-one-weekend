use std::cell::RefCell;
use std::rc::Rc;

use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Lambertian, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone)]
pub(crate) struct HitRecord {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) material_ptr: Rc<RefCell<dyn MaterialTrait>>,
    pub(crate) t: f64,
    pub(crate) front_face: bool
}

impl HitRecord {
    pub(crate) fn new(point: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        let material_ground = Rc::new(RefCell::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0)}));

        HitRecord { point, normal, t, front_face, material_ptr: material_ground }
    }
    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {  outward_normal } else { -outward_normal } }

}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool;
}