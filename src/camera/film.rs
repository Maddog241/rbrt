use super::pixel::Pixel;
use cgmath::Point2;

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Film {
    pub resolution: Point2<usize>,
    pub pixels: Vec<Pixel>,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Pixel::new(1.0, 0.0, 0.0); width * height];
        Film {
            resolution: Point2::new(width, height),
            pixels,
        }
    }

    pub fn record(&mut self, i: usize, j: usize, pixel: Pixel) {
        self.pixels[i * self.resolution.x + j] = pixel;
    }

    pub fn write_to_image(&self, filename: &str) {
        let image = File::create(filename).unwrap();
        let mut writer = BufWriter::new(image);

        let header = String::from("P3\n")
            + &self.resolution.x.to_string()
            + " "
            + &self.resolution.y.to_string()
            + "\n255\n";
        writer.write(header.as_bytes()).unwrap();

        for i in 0..self.resolution.y {
            for j in 0..self.resolution.x {
                let rgb = &self.pixels[i * self.resolution.x + j].to_u8();
                let buf = rgb[0].to_string()
                    + " "
                    + &rgb[1].to_string()
                    + " "
                    + &rgb[2].to_string()
                    + "\n";
                writer.write(buf.as_bytes()).unwrap();
            }
        }
    }
}
