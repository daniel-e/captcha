extern crate captcha;

use captcha::{by_name, gen, CaptchaName, Difficulty};
use std::path::Path;

fn main() {
    gen(Difficulty::Easy)
        .save(Path::new("captcha_random_easy.png"))
        .expect("save failed");
    gen(Difficulty::Medium)
        .save(Path::new("captcha_random_medium.png"))
        .expect("save failed");
    gen(Difficulty::Hard)
        .save(Path::new("captcha_random_hard.png"))
        .expect("save failed");

    by_name(Difficulty::Easy, CaptchaName::Amelia)
        .save(Path::new("captcha_amelia_easy.png"))
        .unwrap();
    by_name(Difficulty::Medium, CaptchaName::Amelia)
        .save(Path::new("captcha_amelia_medium.png"))
        .unwrap();
    by_name(Difficulty::Hard, CaptchaName::Amelia)
        .save(Path::new("captcha_amelia_hard.png"))
        .unwrap();

    by_name(Difficulty::Easy, CaptchaName::Lucy)
        .save(Path::new("captcha_lucy_easy.png"))
        .unwrap();
    by_name(Difficulty::Medium, CaptchaName::Lucy)
        .save(Path::new("captcha_lucy_medium.png"))
        .unwrap();
    by_name(Difficulty::Hard, CaptchaName::Lucy)
        .save(Path::new("captcha_lucy_hard.png"))
        .unwrap();

    by_name(Difficulty::Easy, CaptchaName::Mila)
        .save(Path::new("captcha_mila_easy.png"))
        .unwrap();
    by_name(Difficulty::Medium, CaptchaName::Mila)
        .save(Path::new("captcha_mila_medium.png"))
        .unwrap();
    by_name(Difficulty::Hard, CaptchaName::Mila)
        .save(Path::new("captcha_mila_hard.png"))
        .unwrap();
}
