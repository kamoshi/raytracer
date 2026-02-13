use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    let r = (r * 255.999) as u8;
    let g = (g * 255.999) as u8;
    let b = (b * 255.999) as u8;

    println!("{} {} {}", r, g, b);
}
