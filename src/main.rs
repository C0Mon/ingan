use std::fs::File;

use ingan::{format::{Colour, Ppm}, math::Vec3};

fn main() {

    // Image height and type
    let mut image: Ppm = Ppm::new("P3", 400, 400, 255);



    // Triangle
    
    for i in 1..image.height {
        for j in 1..image.width {
            let pixel: Colour = Colour::new(
                j as f64 / (image.width - 1) as f64,
                i as f64/ (image.height - 1) as f64,
                0.0
            );
            image.set_pixel(j, i, pixel);
        }
    }

    let file: File = File::create("image.ppm").expect("Failed to create image.ppm");
    let _ = image.write_image(file);
}

