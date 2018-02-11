use std;
use image;
use ::color::Color;

pub struct Bitmap {
    width: u32,
    height: u32,

    buffer: Vec<Color>,
}


impl Bitmap {
    pub fn new(width: u32, height: u32) -> Bitmap {
        Bitmap {
            width,
            height,

            buffer: vec![Color::zero(); (width * height) as usize],
        }
    }


    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[(x + y * self.width) as usize] = color;
        }
    }


    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut buf = Vec::with_capacity(self.buffer.len() * 4);

        for color in self.buffer.iter() {
            buf.extend_from_slice(&color.as_bytes());
        }

        image::save_buffer(
            path,
            buf.as_slice(),
            self.width,
            self.height,
            image::ColorType::RGBA(8)
        )
    }
}

