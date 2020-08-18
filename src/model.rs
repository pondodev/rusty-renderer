#![allow(dead_code)]
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Vert {
    x: f64,
    y: f64,
    z: f64,
}

impl Vert {
    fn new(line: &str) -> Self {
        let coords = line.split(" ").collect::<Vec<&str>>();
        Vert {
            x: coords[1].parse().unwrap(),
            y: coords[2].parse().unwrap(),
            z: coords[3].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Face {
    verts: [Vert; 3]
}

impl Face {
    fn new(line: &str, verts: &Vec<Vert>) -> Self {
        // we ignore the texture/normal coords, just want mesh data for now
        let data = line.split(" ").collect::<Vec<&str>>();
        let mut vert_indexes: [usize; 3] = [0; 3];
        for i in 1..=3 {
            vert_indexes[i - 1] = data[i].split("/")
                .collect::<Vec<&str>>()[0]
                .parse().unwrap();
        }

        Face {
            verts: [
                verts[vert_indexes[0] - 1],
                verts[vert_indexes[1] - 1],
                verts[vert_indexes[2] - 1],
            ]
        }
    }
}

pub struct Model {
    pub verts: Vec<Vert>,
    pub faces: Vec<Face>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let f = fs::File::open(path).unwrap();
        let f = BufReader::new(f);

        let mut verts: Vec<Vert> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for line in f.lines() {
            let line = line.unwrap();
            // read in line if it is not empty
            if line.len() != 0 {
                let mut x = 1;
                for (i, &char) in line.as_bytes().iter().enumerate() {
                    if char == b' ' {
                        x = i;
                        break;
                    }
                };

                match &line[0..x] {
                    "v"  => verts.push(Vert::new(&line)),
                    "f"  => faces.push(Face::new(&line, &verts)),
                    "vt" => (), // TODO: texture coords
                    "vn" => (), // TODO: vertex normals
                    "vp" => (), // TODO: ???
                    "l"  => (), // TODO: line element ?
                    "g"  => (), // TODO: face group
                    "s"  => (), // TODO: smoothing group? maybe???
                    "#"  => (), // comment
                    _    => println!("uncaught line: {}", line)
                }
            }
        }

        Model {verts, faces}
    }
}