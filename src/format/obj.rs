use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use rand::Rng;

use crate::{format::Colour, math::{Transform, Vec3}};


#[derive(Clone)]
pub struct Obj {
    pub triangle_points: Vec<Vec3>,
    pub triange_colours: Vec<Colour>,
    pub transform: Transform,
}

impl Obj {
    pub fn new() -> Self {
        Self { 
        triangle_points: Vec::new(), 
        triange_colours: Vec::new(), 
        transform: Transform::default() 
    }
    }

    pub fn get_transformed_point(&self, index: usize) -> Vec3 {
        self.triangle_points[index] + self.transform.position
    }

    pub fn read_obj_file(&mut self, path: &str) {

        let file_path = Path::new(path);

        let file = File::open(file_path).expect("Error reading file");
        let reader = BufReader::new(file);

        println!("Reading file line-by-line:\n");

        let mut all_points: Vec<Vec3> = Vec::new();

        // Read each line form the file
        for line_result in reader.lines() {
            let line = line_result.expect("Error reading line");
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
        let mut rng = rand::rng();
        for _i in 0..self.triangle_points.len() {
            self.triange_colours.push(Colour::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)));
        }
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