extern crate captcha;

use captcha::samples::{self, Difficulty};
use std::path::Path;

fn main() {
    samples::gen(Difficulty::Easy).save(Path::new("captcha_easy.png")).expect("save failed");
    samples::gen(Difficulty::Medium).save(Path::new("captcha_medium.png")).expect("save failed");
    samples::gen(Difficulty::Hard).save(Path::new("captcha_hard.png")).expect("save failed");
}
