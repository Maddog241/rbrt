use cgmath::Point2;

use crate::spectrum::Spectrum;

use super::{Texture, mapping::TextureMapping2D};

use image::{io::Reader as ImageReader, ImageBuffer, Rgb};

pub struct Texels {
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    resolution: Point2<usize>,
}

impl Texels {
    pub fn new(filename: &str) -> Self {
        let img = ImageReader::open(filename)
            .expect("failed to open image file")
            .decode()
            .expect("failed to read the image")
            .to_rgb8();

        let resolution = Point2::new(img.width() as usize, img.height() as usize);

        Self {
            img,
            resolution
        }
    }

    fn eval(&self, st: Point2<f64>) -> Spectrum {
        // st in range [0, 1] x [0, 1]
        let x = (st[0] * self.resolution[0] as f64) as u32;
        let y = (st[1] * self.resolution[1] as f64) as u32;
        let rgb = self.img.get_pixel(x, y);

        Self::to_spectrum(rgb)
    }

    fn to_spectrum(rgb: &Rgb<u8>) -> Spectrum {
        let r = rgb[0] as f64 / 256.0;
        let g = rgb[1] as f64 / 256.0;
        let b = rgb[2] as f64 / 256.0;

        Spectrum::new(r*r, g*g, b*b)
    }
}

pub struct ImageTexture {
    map: Box<dyn TextureMapping2D>,
    texels: Texels,
}

impl ImageTexture {
    pub fn new(map: Box<dyn TextureMapping2D>, texels: Texels) -> Self {
        ImageTexture { map, texels}
    }
}

impl Texture<Spectrum> for ImageTexture {
    fn evaluate(&self, isect: &crate::geometry::interaction::SurfaceInteraction) -> Spectrum {
        let st = self.map.map(isect);
        // check if the st value is outside [0, 1] range

            // in the range
        let texel_val: Spectrum = self.texels.eval(st);
        texel_val
    }
}