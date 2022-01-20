use crate::primatives::Point;

pub struct PixelPoint {
    pub x: i32,
    pub y: i32
}

impl PixelPoint {
    pub fn from_point(point: &Point, image: &Image) -> Self {
        PixelPoint {
            x: (point.x * image.width as f32).floor() as i32,
            y: (point.y * image.height as f32).floor() as i32
        }
    } 
}

pub struct Image {
    pub image_data: Vec<u8>,
    pub width: usize,
    pub height: usize
}

impl Image {
    // @todo: make these take pixel points. 
    pub fn pixel_at(self: &Self, x: usize, y: usize) -> Result<[u8; 3], String> {

        if x > self.width {
            return Err(String::from("Indexing off the side of the image. "));
        }

        let index = Image::position_to_index_into_array(x, y, self.width);
        if index >= self.image_data.len() {
            return Err(String::from("Attempting to index outside the bounds of this image. "));
        }

        let r = self.image_data[index];
        let g = self.image_data[index + 1];
        let b = self.image_data[index + 2];

        Ok([r, g, b])
    }

    pub fn set_pixel(self: &mut Self, x: usize, y: usize, color: &[u8; 3]) -> Result<(), String> {

        if x > self.width {
            return Err(String::from("Indecing off the side of the image. "));
        }

        let index = Image::position_to_index_into_array(x, y, self.width);
        if index >= self.image_data.len() {
            return Err(String::from("Attempting to index outside the bounds of this image. "));
        }

        self.image_data[index] = color[0];
        self.image_data[index + 1] = color[1];
        self.image_data[index + 2] = color[2];

        Ok(())
    }

    fn position_to_index_into_array(x: usize, y: usize, width: usize) -> usize {
        (y * width + x) * 3
    }
}