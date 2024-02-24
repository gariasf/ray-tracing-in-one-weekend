use std::fs::File;
use std::io::{self, Write};

use crate::color;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_float};
use crate::vec3::{cross, Point3, random_in_unit_disk, unit_vector, Vec3};

pub(crate) struct Camera {
    // Public
    pub(crate) aspect_ratio: f64,
    // Ratio of image width over height
    pub(crate) image_width: f64,
    // Rendered image width in pixel count
    pub(crate) samples_per_pixel: i32,
    // Count of random samples for each pixel
    pub(crate) max_depth: i32, // Maximum number of ray bounces into scene

    pub(crate) vfov: f64,
    // Vertical view angle (field of view)
    pub(crate) look_from: Point3,
    // Point camera is looking from
    pub(crate) look_at: Point3,
    // Point camera is looking at
    pub(crate) vup: Vec3, // Camera-relative "up" direction

    pub(crate) defocus_angle: f64,
    // Variation angle of rays through each pixel
    pub(crate) focus_distance: f64, // Distance from camera lookfrom point to plane of perfect focus


    // Private
    image_height: i32,
    // Rendered image height
    center: Point3,
    // Camera center
    pixel00_loc: Point3,
    // Location of pixel 0, 0
    pixel_delta_u: Vec3,
    // Offset to pixel to the right
    pixel_delta_v: Vec3,
    // Offset to pixel below
    u: Vec3,
    // Camera frame basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: f64,
        samples_per_pixel: i32,
        max_depth: i32, vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
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

        self.center = self.look_from;

        // Determine viewport dimensions.
        // let focal_length: f64 = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.vfov);
        let height = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * height * self.focus_distance;
        let viewport_width: f64 = viewport_height * (self.image_width / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = unit_vector(self.look_from - self.look_at);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v: Vec3 = viewport_height * -self.v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 = self.center - (self.focus_distance * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_distance * (degrees_to_radians(self.defocus_angle) / 2.0).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    fn get_ray(&self, u: i32, v: i32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center: Vec3 = self.pixel00_loc + (u as f64 * self.pixel_delta_u + v as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + Self::pixel_sample_square(self);
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { Self::defocus_disk_sample(self) };
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time: f64 = random_float();
        return Ray::new(ray_origin, ray_direction, ray_time);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk
        let point = random_in_unit_disk();
        return self.center + (point[0] * self.defocus_disk_u + point[1] * self.defocus_disk_v);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin
        let px = -0.5 + random_float();
        let py = -0.5 + random_float();
        return px * self.pixel_delta_u + py * self.pixel_delta_v;
    }

    fn ray_color(ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exerted the maximum depth, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record: HitRecord = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);

        if world.hit(ray, Interval::with_bounds(0.001, f64::INFINITY), &mut hit_record) {
            // let direction = hit_record.normal + random_unit_vector();
            // let ray: Ray = Ray::new(hit_record.point, direction);
            let hit: Option<(Color, Ray)> = hit_record.material_ptr.borrow().scatter(ray, &hit_record);
            if hit != None {
                let (attenuation, scattered) = hit.unwrap();
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction: Vec3 = unit_vector(ray.direction());
        const START_VALUE: f64 = 0.5;
        const END_VALUE: f64 = 1.0;
        let delta = START_VALUE * (unit_direction.y() + END_VALUE);

        let white: Color = Color::new(1.0, 1.0, 1.0);
        let blue: Color = Color::new(0.5, 0.7, 1.0);

        return (END_VALUE - delta) * white + delta * blue;
    }
}