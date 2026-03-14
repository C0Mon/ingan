use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use crate::math::Vec3;


pub struct Obj {
    pub triangle_points: Vec<Vec3>
}

impl Obj {
    pub fn new() -> Self {
        Self { triangle_points: Vec::new() }
    }

    pub fn read_obj_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {

        let file_path = Path::new(path);

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        println!("Reading file line-by-line:\n");

        let mut all_points: Vec<Vec3> = Vec::new();

        // Read each line form the file
        for line_result in reader.lines() {
            let line = line_result?;
            let line = line.trim();

            let start_code = &line[..2];

            if start_code == "v " {
                let vertex = &line[2..].split(' ').filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
                all_points.push(Vec3::new(vertex[0], vertex[1], vertex[2]));
            }
            else if start_code == "f " {
                let face_index_groups = line[2..].split(' ').collect::<Vec<_>>();

                for i in 0..face_index_groups.len() {
                    let index_group = face_index_groups[i].split('/').filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>();
                    let point_index = index_group[0] - 1;
                    if i >= 3 { 
                        self.triangle_points.push(self.triangle_points[self.triangle_points.len() - (3 * i - 6)]); 
                        self.triangle_points.push(self.triangle_points[self.triangle_points.len() - 2]); 
                    }
                    self.triangle_points.push(all_points[point_index]);
                }
            }
        }
        Ok(())
    }

    pub fn print_points(&self) {
        for i in 0..self.triangle_points.len() {
            if i % 3 == 0 {
                print!("\n");
            }
            print!("{}  |   ", self.triangle_points[i]);
        }
    }
}