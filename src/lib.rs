extern crate image;
extern crate rand;
extern crate serde_json;
extern crate base64;
extern crate lodepng;

pub mod filters;
mod images;
mod fonts;

use filters::Filter;
use images::{Image, Pixl};
use fonts::{Default, Font};

use std::path::Path;
use std::io::Result;
use rand::{thread_rng, Rng};
use std::cmp::{min, max};

#[derive(Clone, Debug)]
pub struct TextArea {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32
}

pub struct Captcha {
    img: Image,
    font: Box<Font>,
    text_area: TextArea,
}

impl Captcha {
    pub fn new() -> Captcha {
        let w = 400;
        let h = 300;
        Captcha {
            img      : Image::new(w, h),
            font     : Box::new(Default::new()),
            text_area: TextArea {
                left: w / 4, right: w / 4, top: h / 2, bottom: h / 2
            }
        }
    }

    pub fn apply_filter<F: Filter>(&mut self, f: F) -> &mut Self {
        f.apply(&mut self.img);
        self
    }

    pub fn set_font<F: Font + 'static>(&mut self, f: F) -> &mut Self {
        self.font = Box::new(f);
        self
    }

    pub fn save(&self, p: &Path) -> Result<()> { self.img.save(p) }

    fn random_char_as_image(&self) -> Option<Image> {
        let mut rng = thread_rng();
        match rng.choose(&self.font.chars()) {
            None    => None,
            Some(c) => {
                match self.font.png(c.clone()) {
                    None    => None,
                    Some(p) => Image::from_png(p)
                }
            }
        }
    }

    pub fn add_char(&mut self) -> &mut Self {
        match self.random_char_as_image() {
            Some(i) => {
                let x = self.text_area.right;
                let y = (self.text_area.bottom + self.text_area.top) / 2 - i.height() / 2;
                self.img.add_image(x, y, &i);

                self.text_area.top    = min(self.text_area.top, y);
                self.text_area.right  = x + i.width() - 1;
                self.text_area.bottom = max(self.text_area.bottom, y + i.height() - 1);
            },
            _ => { }
        }

        self
    }

    pub fn add_text_area(&mut self) -> &mut Self {
        for y in self.text_area.top..self.text_area.bottom {
            self.img.put_pixel(self.text_area.left, y, Pixl::red());
            self.img.put_pixel(self.text_area.right, y, Pixl::red());
        }
        for x in self.text_area.left..self.text_area.right {
            self.img.put_pixel(x, self.text_area.top, Pixl::red());
            self.img.put_pixel(x, self.text_area.bottom, Pixl::red());
        }
        self
    }

    pub fn text_area(&self) -> TextArea {
        self.text_area.clone()
    }

    pub fn extract(&mut self, area: TextArea) -> &mut Self {
        let w = area.right - area.left + 1;
        let h = area.bottom - area.top + 1;
        let mut i = Image::new(w, h);
        for (y, iy) in (area.top..area.bottom + 1).zip(0..h + 1) {
            for (x, ix) in (area.left..area.right + 1).zip(0..w + 1) {
                i.put_pixel(ix, iy, self.img.get_pixel(x, y));
            }
        }
        self.img = i;
        self
    }

    pub fn view(&mut self, w: u32, h: u32) -> &mut Self {
        let mut a = self.text_area();
        a.left   = (a.right + a.left) / 2 - w / 2;
        a.right  = a.left + w;
        a.top    = (a.bottom + a.top) / 2 - h / 2;
        a.bottom = a.top + h;
        self.extract(a);
        // TODO update text area
        self
    }

    pub fn add_chars(&mut self, n: u32) -> &mut Self {
        for _ in 0..n {
            self.add_char();
        }
        self
    }

    pub fn as_png(self) -> Option<Vec<u8>> {
        self.img.as_png()
    }
}

#[cfg(test)]
mod tests {
    use Captcha;
    use filters::{Noise, Grid, Cow, Dots, Wave, Direction};
    use fonts::Default;

    use std::path::Path;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn it_works() {
        let mut c = Captcha::new();
        c.set_font(Default::new())
            .add_char()
            .add_char()
            .add_char()
            .apply_filter(Noise::new(0.1))
            .apply_filter(Grid::new(20, 10))
            .add_text_area();
            //.apply_filter(Cow::new().min_radius(5).max_radius(20))
            //.apply_filter(Dots::new(40))
            //.apply_filter(Wave::new(2.0, 30.0).direction(Direction::VERTICAL))
            //.apply_filter(Wave::new(2.0, 20.0).direction(Direction::HORIZONTAL))
        let a = c.text_area();
        c.extract(a)
            .save(Path::new("/tmp/a.png"))
            .expect("save failed");

        let data = c.as_png().expect("no png");
        let mut f = File::create(Path::new("/tmp/b.png")).expect("err");
        f.write(&data);

    }
}
