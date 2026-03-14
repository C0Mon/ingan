use std::fs::File;

use ingan::{format::{Colour, Ppm}, math::{Vec3, Vec2}};

fn main() {

    // Image height and type
    let mut image: Ppm = Ppm::new("P3", 400, 400, 255);



    // Triangle
    let a = Vec2::new(0.2 * image.width as f64, 0.2 * image.height as f64);
    let b = Vec2::new(0.7 * image.width as f64, 0.4 * image.height as f64);
    let c = Vec2::new(0.4 * image.width as f64, 0.8 * image.height as f64);

    for i in 1..image.height {
        for j in 1..image.width {
            let p = Vec2::new(j as f64, i as f64);
            let inside = p.point_in_triangle(a, b, c);
            if inside {
                image.set_pixel(j, i, Colour::new(0.0, 0.0, 1.0))
            };
        }
    }

    let file: File = File::create("image.ppm").expect("Failed to create image.ppm");
    let _ = image.write_image(file);
}

