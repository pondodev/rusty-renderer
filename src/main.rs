mod model;
mod vert;
mod face;

use model::Model;
use vert::Vert;
use face::Face;

use image;
use image::{ImageBuffer, Rgb};
use std::mem::swap;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 800;

fn main() {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT);

    let mut model = Model::new("models/cube.obj");

    for i in 0..model.faces.len() {
        draw_triangle(&mut model.faces[i], &mut imgbuf, [255, 255, 255]);
    }

    // flip vertically so that (0, 0) is in the bottom left corner
    image::imageops::flip_vertical_in_place(&mut imgbuf);

    imgbuf.save("output.png").expect("couldn't write to image");
}

fn draw_pixel(x: u32, y: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    if x < IMAGE_WIDTH && y < IMAGE_HEIGHT {
        let pixel = imgbuf.get_pixel_mut(x, y);
        *pixel = image::Rgb(colour);
    }
}

fn draw_line(v0: Vert, v1: Vert, mut imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    // get the screen space coords to draw to
    let mut x0 = ((v0.x + 1.0) * IMAGE_WIDTH as f64 / 2.0) as i32;
    let mut y0 = ((v0.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0) as i32;
    let mut x1 = ((v1.x + 1.0) * IMAGE_WIDTH as f64 / 2.0) as i32;
    let mut y1 = ((v1.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0) as i32;

    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        steep = true;
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    for x in x0..=x1 {
        let t = (x - x0) as f64 / (x1 - x0) as f64;
        let y = y0 as f64 * (1.0 - t) + y1 as f64 * t;
        if steep {
            draw_pixel(y as u32, x as u32, &mut imgbuf, colour);
        } else {
            draw_pixel(x as u32, y as u32, &mut imgbuf, colour);
        }
    }
}

fn draw_triangle(face: &mut Face, mut imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    // draw edges
    for i in 0..3 {
        // get the two verts we wish to draw between
        let v0 = face.verts[i];
        let v1 = face.verts[(i + 1) % 3];

        draw_line(v0, v1, &mut imgbuf, colour);
    }

    // TODO: fill faces
    // sort verts ascending by y coord
    face.sort_asc_y();

    // fill entire tri if it is already flat top/bottom...
    if face.verts[1].y == face.verts[2].y {
        fill_tri_flat_bottom(face);
    } else if face.verts[0].y == face.verts[1].y {
        fill_tri_flat_top(face);
    } else { // ...otherwise it's splittin time
        let v0 = face.verts[0];
        let v1 = face.verts[1];
        let v2 = face.verts[2];
        let v3 = Vert{
            x: v0.x + ((v1.y - v0.y) / (v2.x - v0.x)),
            y: v1.y,
            z: 0.0,
        };

        let flat_bottom = Face {
            verts: [v0, v1, v3],
        };
        let flat_top = Face {
            verts: [v1, v3, v2],
        };
        fill_tri_flat_bottom(&flat_bottom);
        fill_tri_flat_top(&flat_top);
    }
}

fn fill_tri_flat_bottom(face: &Face) {
    // TODO: actually write this lmao
    let height = face.verts[1].y - face.verts[0].y;
}

fn fill_tri_flat_top(face: &Face) {
    // TODO: actually write this lmao
    let height = face.verts[2].y - face.verts[0].y;
}
