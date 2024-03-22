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
use filters::{Cow, Dots, Grid, Noise, Wave};
use rand::{thread_rng, Rng};
use {Captcha, Geometry};

const WIDTH: u32 = 220;
const HEIGHT: u32 = 120;

/// The difficulty of a CAPTCHA.
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Names of predefined CAPTCHAs.
pub enum CaptchaName {
    Amelia,
    Lucy,
    Mila,
}

static CAPTCHA_FUNCTIONS: &[fn(Difficulty) -> Captcha] =
    &[captcha_amelia, captcha_lucy, captcha_mila];

/// Creates a random CAPTCHA with the given difficulty.
///
/// The resulting CAPTCHA has a size of 220x120 pixels and contains between 4 and 6
/// character (including).
///
/// If you need more flexibility please have a look at [`Captcha`](../struct.Captcha.html).
pub fn gen(d: Difficulty) -> Captcha {
    let n = thread_rng().gen::<usize>() % CAPTCHA_FUNCTIONS.len();
    CAPTCHA_FUNCTIONS[n](d)
}

// TODO document easy/medium/hard

/// Creates a predefined CAPTCHA by its name.
///
/// The CAPTCHAs look similar to the following ones:
///
/// <div style="display: table">
/// <div style="display: table-row; background-color: #dddddd">
///   <div style="display: table-cell; text-align: center; padding: 10px;">Name</div>
///   <div style="display: table-cell; text-align: center; padding: 10px;">Easy</div>
///   <div style="display: table-cell; text-align: center; padding: 10px;">Medium</div>
///   <div style="display: table-cell; text-align: center; padding: 10px;">Hard</div>
/// </div>
///
/// <div style="display: table-row;">
///   <div style="display: table-cell; vertical-align: top; padding: 3px;"></div>
///   <div style="display: table-cell; vertical-align: top; padding: 3px;"></div>
///   <div style="display: table-cell; vertical-align: top; padding: 3px;"></div>
///   <div style="display: table-cell; vertical-align: top; padding: 3px;"></div>
/// </div>
///
/// <div style="display: table-row;">
///   <div style="display: table-cell; vertical-align: top; padding-left: 3px;">Amelia</div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_amelia_easy.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_amelia_medium.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_amelia_hard.png">
///   </div>
/// </div>
///
/// <div style="display: table-row;">
///   <div style="display: table-cell; vertical-align: top; padding-left: 3px;">Lucy</div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_lucy_easy.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_lucy_medium.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_lucy_hard.png">
///   </div>
/// </div>
///
/// <div style="display: table-row;">
///   <div style="display: table-cell; vertical-align: top; padding-left: 3px;">Mila</div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_mila_easy.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_mila_medium.png">
///   </div>
///   <div style="display: table-cell; padding-left: 3px;">
///     <img src="https://github.com/daniel-e/captcha/raw/master/doc/captcha_mila_hard.png">
///   </div>
/// </div>
///
/// </div>
///
pub fn by_name(d: Difficulty, t: CaptchaName) -> Captcha {
    match t {
        CaptchaName::Amelia => captcha_amelia(d),
        CaptchaName::Lucy => captcha_lucy(d),
        CaptchaName::Mila => captcha_mila(d),
    }
}

// -------------------------------------------

fn rnd() -> u32 {
    thread_rng().gen_range(4..7)
}

fn captcha_amelia(d: Difficulty) -> Captcha {
    let mut c = Captcha::new();
    c.add_random_chars(rnd());
    match d {
        Difficulty::Easy => c
            .apply_filter(Noise::new(0.2))
            .apply_filter(Grid::new(8, 8)),
        Difficulty::Medium => c
            .apply_filter(Noise::new(0.3))
            .apply_filter(Grid::new(6, 6)),
        Difficulty::Hard => c
            .apply_filter(Noise::new(0.5))
            .apply_filter(Grid::new(4, 4)),
    };
    c.apply_filter(Wave::new(2.0, 10.0)).view(WIDTH, HEIGHT);
    match d {
        Difficulty::Easy => c.apply_filter(Dots::new(10).max_radius(7).min_radius(3)),
        Difficulty::Medium => c.apply_filter(Dots::new(15).max_radius(7).min_radius(4)),
        Difficulty::Hard => c.apply_filter(Dots::new(20).max_radius(7).min_radius(5)),
    };
    c
}

fn captcha_lucy(d: Difficulty) -> Captcha {
    let (n, g) = match d {
        Difficulty::Easy => (0.1, 8),
        Difficulty::Medium => (0.4, 6),
        Difficulty::Hard => (0.6, 4),
    };

    let mut c = Captcha::new();
    c.add_random_chars(rnd())
        .apply_filter(Noise::new(n))
        .apply_filter(Grid::new(g, g))
        .view(WIDTH, HEIGHT);
    c
}

fn captcha_mila(d: Difficulty) -> Captcha {
    let mut c = Captcha::new();
    c.add_random_chars(rnd());
    match d {
        Difficulty::Easy => c.apply_filter(Noise::new(0.2)),
        Difficulty::Medium => c.apply_filter(Noise::new(0.3)),
        Difficulty::Hard => c.apply_filter(Noise::new(0.5)),
    };
    c.apply_filter(Wave::new(2.0, 20.0))
        .view(WIDTH, HEIGHT)
        .apply_filter(
            Cow::new()
                .min_radius(40)
                .max_radius(50)
                .circles(1)
                .area(Geometry::new(40, 150, 50, 70)),
        );
    c
}

//fn hard() -> Captcha {
//    let mut c = Captcha::new();
//    c.add_chars(rnd())
//        .apply_filter(Noise::new(0.4))
//        .apply_filter(Wave::new(2.0, 20.0).horizontal())
//        .apply_filter(Wave::new(2.0, 20.0).vertical())
//        .view(WIDTH, HEIGHT)
//        .apply_filter(Dots::new(15));
//    c
//}
