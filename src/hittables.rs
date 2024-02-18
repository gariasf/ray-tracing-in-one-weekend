use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // Constructs a new, empty `HittableList`.
    pub(crate) fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    // Constructs a `HittableList` with an initial object.
    fn with_object(object: Box<dyn Hittable>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    // Clears all objects from the list.
    fn clear(&mut self) {
        self.objects.clear();
    }

    // Adds an object to the list.
    pub(crate) fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(ray, Interval::with_bounds(interval.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
