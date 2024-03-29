use std::{ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign}, process::exit};

use crate::camera::pixel::Pixel;

pub type Spectrum = RGBSpectrum;

#[derive(Debug, Clone, Copy)]
pub struct RGBSpectrum {
    pub r: f64, 
    pub g: f64, 
    pub b: f64,
}

impl RGBSpectrum {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGBSpectrum { r, g, b }
    }

    pub fn black() -> RGBSpectrum {
        RGBSpectrum::new(0.0, 0.0, 0.0)
    }

    pub fn is_black(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    pub fn to_pixel(&self) -> Pixel {
        check_invalid(self);
        // gamma correction 2.0
        let r = self.r.sqrt();
        let g = self.g.sqrt();
        let b = self.b.sqrt();
        
        Pixel::new(r, g, b)
    }

    pub fn sum(&self) -> f64 {
        self.r + self.g + self.b
    }

    pub fn tone_mapping(&self) -> RGBSpectrum {
        RGBSpectrum::new(
            self.r,
            self.g,
            self.b,
        )
    }

    pub fn skyblue(t: f64) -> RGBSpectrum {
        (1.0-t) * RGBSpectrum::new(1.0, 1.0, 1.0) + t * RGBSpectrum::new(0.5, 0.7, 1.0)
    }

    pub fn contain_nan(&self) -> bool {
        for elem in [self.r, self.g, self.b] {
            if elem.is_nan() {
                return true;
            }
        }

        false
    }
}

fn check_invalid(radiance: &Spectrum) {
    for elem in [radiance.r, radiance.g, radiance.b] {
        if elem.is_nan() || elem < 0.0 {
            println!("error: invalid radiance value {:?}", radiance);
            exit(1);
        }
    }
}

impl Add<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn add(self, rhs: RGBSpectrum) -> Self::Output {
        RGBSpectrum {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        } 
    }
}

impl AddAssign<RGBSpectrum> for RGBSpectrum {
    fn add_assign(&mut self, rhs: RGBSpectrum) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b; 
    }
}

impl Sub<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn sub(self, rhs: RGBSpectrum) -> Self::Output {
        RGBSpectrum {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        } 
    }
}

impl Mul<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, rhs: RGBSpectrum) -> Self::Output {
        RGBSpectrum {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        } 
    }
}

impl Mul<RGBSpectrum> for &RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, rhs: RGBSpectrum) -> Self::Output {
        RGBSpectrum {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign<RGBSpectrum> for RGBSpectrum {
    fn mul_assign(&mut self, rhs: RGBSpectrum) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f64> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, rhs: f64) -> Self::Output {
        RGBSpectrum {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<f64> for &RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, rhs: f64) -> Self::Output {
        RGBSpectrum {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<RGBSpectrum> for f64 {
    type Output = RGBSpectrum;

    fn mul(self, rhs: RGBSpectrum) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for RGBSpectrum {
    type Output = RGBSpectrum;

    fn div(self, rhs: f64) -> Self::Output {
        RGBSpectrum {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl DivAssign<f64> for RGBSpectrum {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs; 
    }
}

impl PartialEq<RGBSpectrum> for RGBSpectrum {
    fn eq(&self, other: &RGBSpectrum) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    #[test] 
    fn test_add() {
        for _i in 0..10 {
            let s1 = Spectrum::new(random(), random(), random());
            let s2 = Spectrum::new(random(), random(), random());

            assert_eq!(s1+s2, Spectrum::new(s1.r + s2.r, s1.g + s2.g, s1.b + s2.b));
        }
   }

   #[test]
    fn test_add_assign() {
        for _i in 0..10 {
            let mut s1 = Spectrum::new(random(), random(), random());
            let s2 = Spectrum::new(random(), random(), random());

            let sum = s1 + s2;
            s1 += s2;

            assert_eq!(s1, sum);
        }
    }

    #[test]
    fn test_sub() {
        for _i in 0..10 {
            let s1 = Spectrum::new(random(), random(), random());
            let s2 = Spectrum::new(random(), random(), random());

            assert_eq!(s1 - s2, Spectrum::new(s1.r - s2.r, s1.g - s2.g, s1.b - s2.b));
        }
   }

    #[test]
    fn test_mul_rgb() {
        for _i in 0..10 {
            let s1 = Spectrum::new(random(), random(), random());
            let s2 = Spectrum::new(random(), random(), random());

            assert_eq!(s1 * s2, Spectrum::new(s1.r * s2.r, s1.g * s2.g, s1.b * s2.b));
        }
    }

    #[test]
    fn test_mul_assign() {
        for _i in 0..10 {
            let mut s1 = Spectrum::new(random(), random(), random());
            let s2 = Spectrum::new(random(), random(), random());
            let s3 = s1 * s2;
            s1 *= s2;
            assert_eq!(s1, s3);
        }
    }

    #[test]
    fn test_mul_f64() {
        for _i in 0..10 {
            let s = Spectrum::new(0.2, 0.1, 0.3);
            let k = 0.3;

            assert_eq!(s * k, Spectrum::new(s.r * k, s.g * k, s.b * k));
            assert_eq!(s * k, k * s);
        }
   }

    #[test]
    fn test_div_f64() {
        for _i in 0..10 {
            let s = Spectrum::new(random(), random(), random());
            let k = 3.0;

            assert_eq!(s / k, Spectrum::new(s.r/k, s.g/k, s.b/k));
        }
    }

    #[test]
    fn test_div_assign() {
        for _i in 0..10 {
            let mut s1 = Spectrum::new(random(), random(), random());
            let k = 7.777;
            let s2 = s1 / k;
            s1 /= k;
            assert_eq!(s1, s2);
        }
    }
}
