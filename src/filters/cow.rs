use rand::{thread_rng, Rng};
use std::cmp::{min, max};
use std::collections::BTreeSet;

use images::Image;
use filters::Filter;

pub struct Cow {
    min_radius: u32,
    max_radius: u32,
    n: u32,
    allow_duplicates: bool
}

impl Cow {
    pub fn new() -> Cow {
        Cow {
            min_radius: 10,
            max_radius: 20,
            n: 3,
            allow_duplicates: true
        }
    }

    pub fn circles(self, n: u32) -> Self {
        Cow { n: n, .. self }
    }

    pub fn min_radius(self, min_radius: u32) -> Self {
        Cow { min_radius: min_radius, .. self }
    }

    pub fn max_radius(self, max_radius: u32) -> Self {
        Cow { max_radius: max_radius, .. self }
    }

    fn get_pixels(x: i32, y: i32, r: i32, i: &mut Image) -> Vec<(u32, u32)> {
        let h = i.height() as i32;
        let w = i.width() as i32;
        let mut v = vec![];

        for py in max(y - r, 0)..min(y + r, h) {
            for px in max(x - r, 0)..min(x + r, w) {
                let dy = y - py;
                let dx = x - px;
                let d = ((dy * dy + dx * dx) as f32).sqrt() as i32;
                if d <= r {
                    v.push((px as u32, py as u32));
                }
            }
        }

        v
    }

    fn invert_pixels(v: &[(u32, u32)], i: &mut Image) {
        for &(x, y) in v {
            let mut p = i.get_pixel(x, y);
            p.invert();
            i.put_pixel(x as u32, y as u32, p);
        }
    }
}

impl Filter for Cow {
    fn apply(&self, i: &mut Image) {
        let mut rng = thread_rng();
        let mut pixels = vec![(rng.gen_range(0, i.width()), rng.gen_range(0, i.height()))];
        let mut set = BTreeSet::new();

        for _ in 0..self.n {
            let p = thread_rng().choose_mut(&mut pixels).expect("failed").clone();

            let r = rng.gen_range(self.min_radius, self.max_radius + 1) as i32;
            let v = Self::get_pixels(p.0 as i32, p.1 as i32, r, i);
            if self.allow_duplicates {
                pixels.extend(&v);
            } else {
                for p in v {
                    if !set.contains(&p) {
                        pixels.push(p);
                        set.insert(p);
                    }
                }
            }
        }

        Self::invert_pixels(&pixels, i);
    }
}