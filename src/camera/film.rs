use crate::spectrum::Spectrum;

use cgmath::Point2;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Mutex;

use image::RgbImage;

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

    pub fn write_to_image(&self, filename: &Path) {
        let mut image = RgbImage::new(self.resolution.x as u32, self.resolution.y as u32);
        let file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(file);
        
        let radiance_map = self.radiance_map.lock().unwrap();

        for i in 0..self.resolution.y {
            for j in 0..self.resolution.x {
                let pixel = radiance_map[i * self.resolution.x + j].to_pixel();
                let rgb = pixel.to_rgb();
                image.put_pixel(j as u32, i as u32, rgb);
            }
        }

        image.write_to(&mut writer, image::ImageOutputFormat::Png).unwrap();
    }
}
