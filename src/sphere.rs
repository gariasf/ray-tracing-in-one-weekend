use std::cell::RefCell;
use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::{Point3, dot, Vec3};
pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
    material_ptr: Rc<RefCell<dyn MaterialTrait>>,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, material_ptr: Rc<RefCell<dyn MaterialTrait>>) -> Self {
        Sphere { center, radius, material_ptr }
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let oc: Point3 = ray.origin() - self.center;
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