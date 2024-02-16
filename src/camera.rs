use std::fs::File;
use std::io::{self, Write};
use crate::{color};
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_float;
use crate::vec3::{Point3, random_in_hemisphere, random_unit_vector, unit_vector, Vec3};

pub(crate) struct Camera {
    // Public
    pub(crate) aspect_ratio: f64,
    pub(crate) image_width: f64,
    pub(crate) samples_per_pixel: i32,
    pub(crate) max_depth: i32,

    // Private
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: i32, max_depth: i32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub(crate) fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        Self::initialize(self);

        let mut file = File::create("output.ppm")?;
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for h in 0..self.image_height {
            eprintln!("Scanlines remaining: {} ", self.image_height - h);

            for w in 0..self.image_width as i32 {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = Self::get_ray(self, w, h);
                    let ray_color: Color = Self::ray_color(&ray, self.max_depth, world);
                    pixel_color = pixel_color + ray_color;
                }

                color::write_color(&mut file, pixel_color, self.samples_per_pixel)?;
            }
        }
        eprintln!("\nDone.");

        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width / self.aspect_ratio) as i32;

        if self.image_height < 1 {
            self.image_height = 1;
        }

        // Camera
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width: f64 = VIEWPORT_HEIGHT * (self.image_width / self.image_height as f64);
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = self.center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, u: i32, v: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center: Vec3 = self.pixel00_loc + (u as f64 * self.pixel_delta_u + v as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + Self::pixel_sample_square(self);
        let ray_origin = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin
        let px = -0.5 + random_float();
        let py = -0.5 + random_float();
        return px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn ray_color(ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exerted the maximum depth, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record: HitRecord = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);

        if world.hit(ray, Interval::with_bounds(0.001, f64::INFINITY), &mut hit_record) {
            let direction = hit_record.normal + random_unit_vector();
            let ray: Ray = Ray::new(hit_record.point, direction);
            return 0.5 * (Self::ray_color(&ray, depth - 1, world));
        }

        let unit_direction: Vec3 = unit_vector(ray.direction());
        const START_VALUE : f64 = 0.5;
        const END_VALUE : f64 = 1.0;
        let delta = START_VALUE * (unit_direction.y() + END_VALUE);

        let white: Color = Color::new(1.0, 1.0, 1.0);
        let blue: Color = Color::new(0.5, 0.7, 1.0);

        return (END_VALUE - delta) * white + delta * blue;
    }
}