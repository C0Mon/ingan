
use std::{fs::File, io::{self, BufWriter, Write}};

use crate::{format::Colour, math::Vec3};
pub struct Ppm {
    pub image_type: String,
    pub width: usize,
    pub height: usize,
    pub max_val: usize,
    pub image: Vec<Vec<Colour>>
}

impl Ppm {
    pub fn new(image_type: &str, width: usize, height: usize, max_val: usize) -> Self {
        let image = vec![vec![Vec3::default(); width]; height];
        Self { image_type: image_type.to_string(), width, height, max_val, image }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, pixel: Vec3) {
        self.image[row][col] = pixel;
    }
    
    pub fn write_buffer(&mut self, buffer: Vec<Vec3>) {
        let mut col: usize;
        let mut row: usize;
        for i in 0..self.width * self.height {
            row = buffer.len() % self.width;
            col = buffer.len() / self.width;
            self.image[row][col] = buffer[i];
        }
    }
    pub fn get_headers(&self) -> String {
        format!(
            "{0}\n{1} {2}\n{3}\n",
            self.image_type, self.width, self.height, self.max_val
        )
    }

    pub fn write_image(&self, file: File) -> io::Result<()> {
        let mut writer = BufWriter::new(file);
        writer.write_all(&(self.get_headers()).into_bytes())?;
        for x in 0..self.height {
            eprint!("\rScanlines remaining: {} ", self.height - x);
            io::stderr().flush().unwrap();
            for y in 0..self.width {
                let pixel: Colour = self.image[x][y];
                pixel.write_colour(&mut writer)?;
            }
        }
        writer.flush()?;
        eprint!("\r\x1b[2KDone\n");
        Ok(())
    }
}
