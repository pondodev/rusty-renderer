mod model;

use crate::model::{Model, Face};

use image;
use image::{ImageBuffer, Rgb};
use std::mem::swap;

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 800;

fn main() {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT);

    let model = Model::new("models/teapot.obj");

    for i in 0..model.faces.len() {
        draw_triangle(&model.faces[i], &mut imgbuf, [255, 255, 255]);
    }

    // flip vertically so that (0, 0) is in the bottom left corner
    image::imageops::flip_vertical_in_place(&mut imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn draw_pixel(x: u32, y: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    if x < IMAGE_WIDTH && y < IMAGE_HEIGHT {
        let pixel = imgbuf.get_pixel_mut(x, y);
        *pixel = image::Rgb(colour);
    }
}

fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
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

fn draw_triangle(face: &Face, mut imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    for i in 0..3 {
        // get the two verts we wish to draw between
        let v0 = face.verts[i];
        let v1 = face.verts[(i + 1) % 3];

        // get the screen space coords to draw to
        let x0 = ((v0.x + 1.0) * IMAGE_WIDTH as f64 / 2.0) as i32;
        let y0 = ((v0.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0) as i32;
        let x1 = ((v1.x + 1.0) * IMAGE_WIDTH as f64 / 2.0) as i32;
        let y1 = ((v1.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0) as i32;

        draw_line(x0, y0, x1, y1, &mut imgbuf, colour);
    }
}
