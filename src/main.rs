use crate::color::{Color, write_color};

mod color;
mod vec3;

fn main() {
    // image

    let width = 256;
    let height = 256;

    // render

    println!("P3\n{width} {height}\n255");

    for y in 0..height {
        eprintln!("Scanline {y}");
        for x in 0..width {
            let r = x as f64 / (width as f64 - 1.0);
            let g = y as f64 / (height as f64 - 1.0);
            let color = Color::new(r, g, 0.0);

            write_color(&color);
        }
    }

    eprintln!("Done")
}
