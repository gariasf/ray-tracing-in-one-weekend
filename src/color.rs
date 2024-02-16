use crate::interval::Interval;
use crate::vec3::{Vec3};

pub(crate) type Color = Vec3;

pub(crate) fn write_color(out: &mut dyn std::io::Write, pixel_color: Color, samples_per_pixel: i32) -> std::io::Result<()> {
    let red  = pixel_color.x();
    let green   = pixel_color.y();
    let blue  = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;

    let normalised_red = red * scale;
    let normalised_green = green * scale;
    let normalised_blue = blue * scale;

    let intensity: Interval = Interval::with_bounds(0.0, 0.999);

    let red_value = 256.0 * intensity.clamp(normalised_red);
    let green_value = 256.0 * intensity.clamp(normalised_green);
    let blue_value =  256.0 * intensity.clamp(normalised_blue);

    writeln!(
        out,
        "{} {} {}",
        red_value,
        green_value,
        blue_value
    )
}
