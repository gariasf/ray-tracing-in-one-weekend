use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, unit_vector, Vec3};


pub(crate) trait MaterialTrait  {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub enum Material {
    Lambertian(Box<Lambertian>),
    Metal(Box<Metal>),
}
impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(material) => material.scatter(ray_in, hit_record),
            Material::Metal(material) => material.scatter(ray_in, hit_record),
        }
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl MaterialTrait for Lambertian {

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl MaterialTrait for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(unit_vector(ray_in.direction()), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected + self.fuzz * random_unit_vector());
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}