use std::cell::RefCell;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialTrait;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    material_ptr: Rc<RefCell<dyn MaterialTrait>>,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, material_ptr: Rc<RefCell<dyn MaterialTrait>>, center_1: Point3, is_moving: bool) -> Self {
        Sphere { center, radius, material_ptr, is_moving, center_vec: center_1 - center }
    }

    pub(crate) fn center(&self, time: f64) -> Point3 {
        if self.is_moving {
            return self.center + time * self.center_vec;
        }
        self.center
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let center: Point3 = if self.is_moving {
            self.center(ray.time())
        } else {
            self.center
        };

        let oc: Point3 = ray.origin() - center;
        let a: f64 = ray.direction().length_squared();
        let half_b: f64 = dot(oc, ray.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        record.t = root;
        record.point = ray.at(record.t);

        let outward_normal: Vec3 = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material_ptr = Rc::clone(&self.material_ptr);

        return true;
    }
}