use crate::ray::{Ray};
use crate::vec3::{Point3, Vec3, dot};

#[derive(Clone, Copy)]
pub(crate) struct HitRecord {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self { point, normal, t, front_face }
    }

    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {  outward_normal } else { -outward_normal } }

}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}