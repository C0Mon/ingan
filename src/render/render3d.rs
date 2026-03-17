use crate::{format::{Colour, Obj}, math::{Interval, Vec2, Vec3}};

pub struct Render3D {
    pub width: usize,
    pub height: usize,
    models: Vec<Obj>,
    pub colour_buffer: Vec<Vec3>
}

impl Render3D {
    pub fn new(width: usize, height: usize) -> Self{
        Self { width, height, colour_buffer: vec![Vec3::new(0.0, 0.0, 0.0); width * height], models: Vec::new()}
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f64, self.height as f64)
    }

    pub fn render_scene(&mut self) {
        let size = self.size();
        let width = self.width;
        let height = self.height;

        self.colour_buffer = vec![Vec3::new(0.0, 0.0, 0.0); width * height];
        for model in &self.models {
            
            // This 'model' now has the UPDATED transform from Python!
            Self::render(&model, &mut self.colour_buffer, size, width, height);
        }
    }

    pub fn add_model(&mut self, model: Obj) {
        self.models.push(model);
    }

    pub fn get_model(&mut self, index: usize) -> &mut Obj{
        &mut self.models[index]
    }

}

impl Render3D {
    pub fn render(model: &Obj, buffer: &mut [Colour], size: Vec2, width: usize, height: usize) {
        for i in (0..model.triangle_points.len()).step_by(3) {
            let a = model.get_transformed_point(i).vertex_to_screen(size);
            let b = model.get_transformed_point(i + 1).vertex_to_screen(size);
            let c = model.get_transformed_point(i + 2).vertex_to_screen(size);
            
            // Triangle bounds
            let min_x = f64::min(f64::min(a.x, b.x), c.x);
            let min_y = f64::min(f64::min(a.y, b.y), c.y);
            let max_x = f64::max(f64::max(a.x, b.x), c.x);
            let max_y = f64::max(f64::max(a.y, b.y), c.y);
            
            // Pixel block covering the triangle bounds
            let x_interval = Interval::new(0.0, (width - 1) as f64);
            let y_interval = Interval::new(0.0, (height - 1) as f64);
            
            let block_start_x = x_interval.clamp(min_x) as usize;
            let block_start_y = y_interval.clamp(min_y) as usize;
            let block_end_x = x_interval.clamp(max_x) as usize;
            let block_end_y = y_interval.clamp(max_y) as usize;
            
            for y in block_start_y..=block_end_y {
                for x in block_start_x..=block_end_x {
                    let pixel = Vec2::new(x as f64, y as f64);
                    if pixel.point_in_triangle(a, b, c) {
                        let index = y * width + x;
                        if index < buffer.len() {
                            buffer[index] = model.triange_colours[i/3];
                        }
                    }
                }
            }
        }
    }
    pub fn get_buffer(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.width * self.height * 3);
        for v in &self.colour_buffer {
            result.push((v.x * 255.0) as u8);
            result.push((v.y * 255.0) as u8);
            result.push((v.z * 255.0) as u8);
        }
        result
    }
}
