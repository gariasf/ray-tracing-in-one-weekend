use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;


    let mut file = File::create("output.ppm")?;
    writeln!(file, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255")?;

    for h in 0..IMAGE_HEIGHT {
        for w in 0..IMAGE_WIDTH {
            let red  = w as f64  / (IMAGE_WIDTH - 1) as f64;
            let green = h as f64  / (IMAGE_HEIGHT - 1) as f64;
            let blue = 0.0f64;

            let normalised_red  = (255.999 * red) as i32;
            let normalised_green   = (255.999 * green) as i32;
            let normalised_blue  = (255.999 * blue) as i32;

            writeln!(file, "{normalised_red} {normalised_green} {normalised_blue}")?;
        }
    }

    Ok(())
}
