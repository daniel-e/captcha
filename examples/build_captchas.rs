extern crate captcha;

use captcha::{Captcha, Geometry};
use captcha::filters::{Cow, Noise, Wave};

use std::path::Path;

fn main() {
    Captcha::new()
        .add_chars(5)
        .apply_filter(Noise::new(0.2))
        .apply_filter(Wave::new(2.0, 20.0))
        .view(220, 120)
        .apply_filter(Cow::new().min_radius(40).max_radius(50).circles(1).area(Geometry::new(40, 150, 50, 70)))
//        .apply_filter(Cow::new().min_radius(20).max_radius(30).circles(1).area(Geometry::new(50, 80, 30, 80)))
        .save(Path::new("captcha.png"))
        .expect("save failed");


}
