use rand::{thread_rng, Rng};

use crate::filters::Filter;
use crate::images::{Image, Pixl};

pub struct Dots {
    n: u32,
    min_radius: u32,
    max_radius: u32,
}

impl Dots {
    pub fn new(n: u32) -> Self {
        Dots {
            n,
            min_radius: 5,
            max_radius: 10,
        }
    }

    pub fn min_radius(self, r: u32) -> Self {
        Dots {
            min_radius: r,
            ..self
        }
    }

    pub fn max_radius(self, r: u32) -> Self {
        Dots {
            max_radius: r,
            ..self
        }
    }
}

impl Filter for Dots {
    fn apply(&self, i: &mut Image) {
        let mut rng = thread_rng();
        for _ in 0..self.n {
            let x = rng.gen_range(0..i.width());
            let y = rng.gen_range(0..i.height());
            let r = rng.gen_range(self.min_radius..self.max_radius + 1);
            i.fill_circle(x, y, r, Pixl::black());
        }
    }
}
