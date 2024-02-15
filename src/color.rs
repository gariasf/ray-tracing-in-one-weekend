use crate::vec3::{Vec3};

pub(crate) type Color = Vec3;

pub(crate) fn write_color(out: &mut dyn std::io::Write, pixel_color: Color) -> std::io::Result<()> {
    let normalised_red  = (255.999 * pixel_color.x()) as i32;
    let normalised_green   = (255.999 * pixel_color.y()) as i32;
    let normalised_blue  = (255.999 * pixel_color.z()) as i32;

    writeln!(
        out,
        "{} {} {}",
        normalised_red ,
        normalised_green,
        normalised_blue
    )
}
