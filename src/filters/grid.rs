use crate::filters::Filter;
use crate::images::{Image, Pixl};

pub struct Grid {
    y_gap: u32,
    x_gap: u32,
}

impl Grid {
    pub fn new(x_gap: u32, y_gap: u32) -> Self {
        Self { x_gap, y_gap }
    }
}

impl Filter for Grid {
    fn apply(&self, i: &mut Image) {
        for y in (0..i.height()).filter(|i| i % self.y_gap == 0) {
            for x in 0..i.width() {
                i.put_pixel(x, y, Pixl::black());
            }
        }
        for x in (0..i.width()).filter(|i| i % self.x_gap == 0) {
            for y in 0..i.height() {
                i.put_pixel(x, y, Pixl::black());
            }
        }
    }
}
