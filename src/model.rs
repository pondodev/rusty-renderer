use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

use crate::vert::Vert;
use crate::face::Face;


pub struct Model {
    pub verts: Vec<Vert>,
    pub faces: Vec<Face>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let f = fs::File::open(path).expect("failed to open file");
        let f = BufReader::new(f);

        let mut verts: Vec<Vert> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for line in f.lines() {
            let line = line.expect("failed to read line in file");
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