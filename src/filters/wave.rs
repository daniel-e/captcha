use std::f64::consts;

use filters::Filter;
use images::Image;

pub enum Direction {
    HORIZONTAL,
    VERTICAL,
}

pub struct Wave {
    f: f64,
    amp: f64,
    d: Direction,
}

impl Wave {
    pub fn new(f: f64, amp: f64) -> Wave {
        Wave {
            f: f,
            amp: amp,
            d: Direction::HORIZONTAL,
        }
    }

    pub fn horizontal(self) -> Wave {
        Wave {
            d: Direction::HORIZONTAL,
            ..self
        }
    }

    pub fn vertical(self) -> Wave {
        Wave {
            d: Direction::VERTICAL,
            ..self
        }
    }

    pub fn direction(self, d: Direction) -> Wave {
        Wave { d: d, ..self }
    }
}

// TODO randomize offset

impl Filter for Wave {
    fn apply(&self, i: &mut Image) {
        let o = i.clone();
        i.clear();
        match self.d {
            Direction::HORIZONTAL => {
                // height of image changes
                for x in 0..i.width() {
                    let f =
                        (x as f64 * 2.0 * consts::PI * self.f / i.width() as f64).sin() * self.amp;
                    for y in 0..i.height() {
                        let ny = y as i32 - f as i32;
                        if ny >= 0 && ny < i.height() as i32 {
                            i.put_pixel(x, ny as u32, o.get_pixel(x, y));
                        }
                    }
                }
            }
            Direction::VERTICAL => {
                for y in 0..i.height() {
                    let f =
                        (y as f64 * 2.0 * consts::PI * self.f / i.width() as f64).sin() * self.amp;
                    for x in 0..i.width() {
                        let nx = x as i32 - f as i32;
                        if nx >= 0 && nx < i.width() as i32 {
                            i.put_pixel(nx as u32, y, o.get_pixel(x, y));
                        }
                    }
                }
            }
        }
    }
}
