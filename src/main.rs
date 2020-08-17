use image;

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT);

    // set pixel at (52, 41) to red
    let pixel = imgbuf.get_pixel_mut(52, 41);
    *pixel = image::Rgb([255 as u8, 0 as u8, 0 as u8]);

    // flip vertically so that (0, 0) is in the bottom left corner
    image::imageops::flip_vertical_in_place(&mut imgbuf);

    imgbuf.save("output.tga").unwrap();
}

