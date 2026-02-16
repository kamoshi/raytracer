mod color;
mod ray;
mod vec3;

use crate::{
    color::{Color, write_color},
    ray::Ray,
    vec3::{Point3, Vec3},
};

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image

    let aspect_ratio = 16.0 / 9.0;
    let image_w = 400;
    let image_h = (image_w as f64 / aspect_ratio) as i32;
    let image_h = if image_h > 0 { image_h } else { 1 };

    // camera

    let focal_length = 1.0;
    let viewport_h = 2.0;
    let viewport_w = viewport_h * (image_w as f64 / image_h as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // viewport edges
    let viewport_u = Vec3::new(viewport_w, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_h, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_w as f64;
    let pixel_delta_v = viewport_v / image_h as f64;

    // calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // render

    println!("P3\n{image_w} {image_h}\n255");

    for row in 0..image_h {
        eprintln!("Scanline {row}");
        for col in 0..image_w {
            let pixel_center =
                pixel00_loc + (col as f64 * pixel_delta_u) + (row as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray);

            write_color(&color);
        }
    }

    eprintln!("Done")
}
