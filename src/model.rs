#![allow(dead_code)]
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Vert {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug)]
pub struct Face {
    verts: [Vert; 3]
}

pub struct Model {
    pub verts: Vec<Vert>,
    pub faces: Vec<Face>
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
                    "v"  => verts.push(Model::parse_vert(&line)),
                    "f"  => faces.push(Model::parse_face(&line, &verts)),
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

    fn parse_vert(line: &str) -> Vert {
        let coords = line.split(" ").collect::<Vec<&str>>();
        Vert {
            x: coords[1].parse().unwrap(),
            y: coords[2].parse().unwrap(),
            z: coords[3].parse().unwrap()
        }
    }

    fn parse_face(line: &str, verts: &Vec<Vert>) -> Face {
        // TODO: actually parse the face lol
        Face {
            verts: [
                Vert {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                Vert {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                Vert {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                }
            ]
        }
    }
}