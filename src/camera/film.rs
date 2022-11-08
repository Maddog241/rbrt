use crate::spectrum::Spectrum;

use super::pixel::Pixel;
use cgmath::{Point2, Vector3};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;

pub struct Film {
    pub resolution: Point2<usize>,
    pub radiance_map: Mutex<Vec<Spectrum>>,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        let radiance_map = vec![Spectrum::new(0.0, 0.0, 0.0); width * height];
        Film {
            resolution: Point2::new(width, height),
            radiance_map: Mutex::new(radiance_map),
        }
    }

    pub fn record(&self, i: usize, j: usize, radiance: Spectrum) {
        let mut radiance_map = self.radiance_map.lock().unwrap();
        radiance_map[i * self.resolution.x + j] += radiance;
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

        let radiance_map = self.radiance_map.lock().unwrap();

        for i in 0..self.resolution.y {
            for j in 0..self.resolution.x {
                let pixel = radiance_map[i * self.resolution.x + j].to_pixel();
                let rgb = &pixel.to_u8();
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
