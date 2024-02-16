mod vec3;
mod color;
mod ray;

use std::fs::File;
use std::io::{self, Write};
use std::ops::Div;
use color::{Color};
use vec3::{Vec3, Point3};
use ray::{Ray};
use crate::vec3::unit_vector;

fn ray_color(r: &Ray) -> Color {
    let unit_direction: Vec3 = unit_vector(r.direction());
    const START_VALUE : f64 = 0.5;
    const END_VALUE : f64 = 1.0;
    let delta = START_VALUE * (unit_direction.y() + END_VALUE);

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);

    return (END_VALUE - delta) * white + delta * blue;
}

fn main() -> io::Result<()> {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400f64;

    let mut image_height: i32 = (IMAGE_WIDTH / ASPECT_RATIO) as i32;

    if(image_height < 1) {
        image_height = 1;
    }

    // Camera
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    let viewport_width: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / IMAGE_WIDTH;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 = camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("output.ppm")?;
    writeln!(file, "P3\n{} {}\n255", IMAGE_WIDTH, image_height)?;

    for h in 0..image_height {
        eprintln!("Scanlines remaining: {} ", image_height - h);

        for w in 0..IMAGE_WIDTH as i32 {
            let pixel_center: Vec3 = pixel00_loc + w as f64 * pixel_delta_u + h as f64 * pixel_delta_v;
            let ray_direction: Vec3 = pixel_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color: Color = ray_color(&ray);

            // let pixel_color = color::Color::new(
            //   w as f64 / (IMAGE_WIDTH - 1) as f64,
            //   h as f64 / (image_height - 1) as f64,
            //  0.0,
            // );
            color::write_color(&mut file, pixel_color)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}
