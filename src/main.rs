mod vec3;
mod color;

use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;


    let mut file = File::create("output.ppm")?;
    writeln!(file, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255")?;
    for h in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {} ", IMAGE_HEIGHT - h);

        for w in 0..IMAGE_WIDTH {
            let pixel_color = color::Color::new(
                w as f64 / (IMAGE_WIDTH - 1) as f64,
                h as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            color::write_color(&mut file, pixel_color)?;
        }
    }
    eprintln!("\nDone.");
    Ok(())
}
