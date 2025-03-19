extern crate captcha;

use captcha::{by_name, generate, CaptchaName, Difficulty};
use std::path::Path;

fn main() {
    for i in 1..=20 {
        let s = format!("/tmp/cc/captcha_{i}.png");
        generate(Difficulty::Hard)
            .save(Path::new(&s))
            .expect("save failed");
    }
}
