use image::ImageResult as Result;
use std::cmp::{max, min};
use std::path::Path;

use image::{load_from_memory, ImageBuffer, Rgb, RgbImage};

#[derive(Clone, Copy)]
pub struct Pixl {
    rgb: [u8; 3],
}

#[derive(Clone)]
pub struct Image {
    img: RgbImage,
}

impl Pixl {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { rgb: [r, g, b] }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn invert(&mut self) {
        self.rgb[0] = 255 - self.rgb[0];
        self.rgb[1] = 255 - self.rgb[1];
        self.rgb[2] = 255 - self.rgb[2];
    }
}

impl Image {
    fn pixel_white() -> Rgb<u8> {
        Rgb::<u8>([255, 255, 255])
    }

    pub fn from_png(v: Vec<u8>) -> Option<Self> {
        load_from_memory(&v).ok().map(|v| Self { img: v.to_rgb8() })
    }

    pub fn new(w: u32, h: u32) -> Self {
        Self {
            img: ImageBuffer::from_pixel(w, h, Self::pixel_white()),
        }
    }

    pub fn set_color(&mut self, color: &[u8; 3]) {
        // TODO: optimize
        for y in 0..self.img.height() {
            for x in 0..self.img.width() {
                let c = *self.img.get_pixel(x, y);
                if c[0] == 0 {
                    // if red channel is 0 we assume it's a black pixel
                    let rgb = Rgb::<u8>(*color);
                    self.img.put_pixel(x, y, rgb);
                }
            }
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, p: Pixl) {
        if x < self.img.width() && y < self.img.height() {
            self.img.put_pixel(x, y, Rgb::<u8>(p.rgb));
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixl {
        let p = *self.img.get_pixel(x, y);
        Pixl {
            rgb: [p[0], p[1], p[2]],
        }
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn save<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        self.img.save(p)
    }

    pub fn fill_circle(&mut self, x: u32, y: u32, r: u32, p: Pixl) {
        let h = self.height();
        let w = self.width();

        for py in max(y as i32 - r as i32, 0)..min(y + r, h) as i32 {
            for px in max(x as i32 - r as i32, 0)..min(x + r, w) as i32 {
                let dy = y as i32 - py;
                let dx = x as i32 - px;
                let d = ((dy * dy + dx * dx) as f32).sqrt() as u32;
                if d <= r {
                    self.put_pixel(px as u32, py as u32, p);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.img = ImageBuffer::from_pixel(self.width(), self.height(), Self::pixel_white())
    }

    pub fn add_image(&mut self, x: u32, y: u32, i: &Image) {
        for iy in 0..i.height() {
            for ix in 0..i.width() {
                self.put_pixel(x + ix, y + iy, i.get_pixel(ix, iy));
            }
        }
    }

    pub fn as_png(&self) -> Option<Vec<u8>> {
        let w = self.img.width() as usize;
        let h = self.img.height() as usize;
        let i = self.img.clone().into_raw();
        lodepng::encode_memory(&i, w, h, lodepng::ColorType::RGB, 8).ok()
    }
}
