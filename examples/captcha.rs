extern crate captcha;

use captcha::{gen, Difficulty};
use std::path::Path;

fn main() {
    gen(Difficulty::Easy).save(Path::new("captcha_easy.png")).expect("save failed");
    gen(Difficulty::Medium).save(Path::new("captcha_medium.png")).expect("save failed");
    gen(Difficulty::Hard).save(Path::new("captcha_hard.png")).expect("save failed");
}
