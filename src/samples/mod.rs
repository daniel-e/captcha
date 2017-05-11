//! Convinient module to create CAPTCHAs.
//!
//! If you are looking for a more flexible approach to create CAPTCHAs please have a look
//! at [`Captcha`](../struct.Captcha.html).
//!
//! # Examples
//!
//! ```
//! # extern crate captcha;
//! use captcha::{gen, Difficulty};
//!
//! # fn main() {
//! gen(Difficulty::Easy).as_png();
//! # }
//! ```
use rand::{Rng, thread_rng};
use Captcha;
use filters::{Noise, Grid, Dots, Wave};

const WIDTH: u32 = 220;
const HEIGHT: u32 = 120;

/// Represents the difficulty of a CAPTCHA.
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

/// Creates a random CAPTCHA with the given difficulty.
///
/// The resulting CAPTCHA has a size of 220x120 pixels and contains between 4 and 6
/// character (including).
///
/// If you need more flexibility please have a look at [`Captcha`](../struct.Captcha.html).
pub fn gen(d: Difficulty) -> Captcha {
    match d {
        Difficulty::Easy   => easy(),
        Difficulty::Medium => medium(),
        Difficulty::Hard   => hard()
    }
}

fn rnd() -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(4, 7)
}

fn easy() -> Captcha {
    let mut c = Captcha::new();
    c.add_chars(rnd())
        .apply_filter(Noise::new(0.1))
        .apply_filter(Grid::new(8, 8))
        .view(WIDTH, HEIGHT);
    c
}

fn medium() -> Captcha {
    let mut c = Captcha::new();
    c.add_chars(5)
        .apply_filter(Noise::new(0.2))
        .apply_filter(Grid::new(8, 8))
        .view(WIDTH, HEIGHT)
        .apply_filter(Dots::new(20).max_radius(7).min_radius(4));
    c
}

fn hard() -> Captcha {
    let mut c = Captcha::new();
    c.add_chars(rnd())
        .apply_filter(Noise::new(0.4))
        .apply_filter(Wave::new(2.0, 20.0).horizontal())
        .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(WIDTH, HEIGHT)
        .apply_filter(Dots::new(15));
    c
}


