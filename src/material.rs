use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_float;
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3};

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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl MaterialTrait for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(ray_in.direction());

        let cos_theta = f64::min(dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
            direction = reflect(unit_direction, hit_record.normal);
        } else {
            direction = refract(unit_direction, hit_record.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit_record.point, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}