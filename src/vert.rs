use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vert {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vert {
    pub fn new(line: &str) -> Self {
        let coords = line.split(" ").collect::<Vec<&str>>();
        Vert {
            x: coords[1].parse().expect("failed to parse vert coords"),
            y: coords[2].parse().expect("failed to parse vert coords"),
            z: coords[3].parse().expect("failed to parse vert coords"),
        }
    }
}

impl Add<Vert> for Vert {
    type Output = Vert;

    fn add(self, other: Vert) -> Self {
        Vert {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vert> for Vert {
    type Output = Vert;

    fn sub(self, other: Vert) -> Self {
        Vert {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vert {
    type Output = Vert;

    fn mul(self, other: f64) -> Self {
        Vert {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
