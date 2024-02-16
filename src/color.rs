use crate::interval::Interval;
use crate::vec3::{Vec3};

pub(crate) type Color = Vec3;

pub(crate) fn linear_to_gamma(color: f64) -> f64 {
    color.sqrt()
}

pub(crate) fn write_color(out: &mut dyn std::io::Write, pixel_color: Color, samples_per_pixel: i32) -> std::io::Result<()> {
    let mut red  = pixel_color.x();
    let mut green   = pixel_color.y();
    let mut blue  = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
     red = red * scale;
     green = green * scale;
     blue = blue * scale;

    // Apply the linear to gamma transform
        red = linear_to_gamma(red);
        green = linear_to_gamma(green);
        blue = linear_to_gamma(blue);

    let intensity: Interval = Interval::with_bounds(0.0, 0.999);

     red = 256.0 * intensity.clamp(red);
     green = 256.0 * intensity.clamp(green);
     blue =  256.0 * intensity.clamp(blue);

    writeln!(
        out,
        "{} {} {}",
        red,
        green,
        blue
    )
}
