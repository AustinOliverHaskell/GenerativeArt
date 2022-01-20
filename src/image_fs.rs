use crate::image::Image;

use std::fs::File;
use std::io::BufWriter;

pub fn write_image(path: &str, image: Image) {
    let file = File::create(path).unwrap();
    let ref mut buff_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buff_writer, image.width as u32, image.height as u32);
    encoder.set_color(png::ColorType::Rgb);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image.image_data).unwrap(); // Save
}