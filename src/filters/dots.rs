use rand::{rng, Rng};

use filters::Filter;
use images::{Image, Pixl};

pub struct Dots {
    n: u32,
    min_radius: u32,
    max_radius: u32,
}

impl Dots {
    pub fn new(n: u32) -> Dots {
        Dots {
            n,
            min_radius: 5,
            max_radius: 10,
        }
    }

    pub fn min_radius(self, r: u32) -> Dots {
        Dots {
            min_radius: r,
            ..self
        }
    }

    pub fn max_radius(self, r: u32) -> Dots {
        Dots {
            max_radius: r,
            ..self
        }
    }
}

impl Filter for Dots {
    fn apply(&self, i: &mut Image) {
        let mut rng = rng();
        for _ in 0..self.n {
            let x = rng.random_range(0..i.width());
            let y = rng.random_range(0..i.height());
            let r = rng.random_range(self.min_radius..self.max_radius + 1);
            i.fill_circle(x, y, r, Pixl::black());
        }
    }
}
