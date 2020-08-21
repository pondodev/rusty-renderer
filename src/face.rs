use crate::vert::Vert;

#[derive(Debug)]
pub struct Face {
    pub verts: [Vert; 3]
}

impl Face {
    pub fn new(line: &str, verts: &Vec<Vert>) -> Self {
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
