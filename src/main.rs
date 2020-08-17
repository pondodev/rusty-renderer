#![allow(dead_code)]
mod model;

use crate::model::Model;

use image;
use image::{ImageBuffer, Rgb};

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

fn main() {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT);

    let model = Model::new("head.obj");
    println!("verts: {:?}", model.verts);
    println!("faces: {:?}", model.faces);

    // flip vertically so that (0, 0) is in the bottom left corner
    image::imageops::flip_vertical_in_place(&mut imgbuf);

    imgbuf.save("output.tga").unwrap();
}

fn draw_pixel(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, colour: [u8; 3]) {
    let pixel = imgbuf.get_pixel_mut(x, y);
    *pixel = image::Rgb(colour);
}

fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, colour: [u8; 3]) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        steep = true;
        let mut temp = x0;
        x0 = y0;
        y0 = temp;

        temp = x1;
        x1 = y1;
        y1 = temp;
    }

    if x0 > x1 {
        let mut temp = x0;
        x0 = x1;
        x1 = temp;

        temp = y0;
        y0 = y1;
        y1 = temp;
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;
    for x in x0..x1 {
        if steep {
            draw_pixel(&mut imgbuf, y as u32, x as u32, colour);
        } else {
            draw_pixel(&mut imgbuf, x as u32, y as u32, colour);
        }

        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 {1} else {-1};
        }
    }
}
