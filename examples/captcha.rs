extern crate captcha;

use captcha::Captcha;
use captcha::filters::{Noise, Grid, Dots, Wave, Direction};

use std::path::Path;

fn easy(p: &Path) {
    let mut c = Captcha::new();
    c.add_char()
        .add_char()
        .add_char()
        .add_char()
        .add_char()
        .apply_filter(Noise::new(0.1))
        .apply_filter(Grid::new(5, 5))
        .view(220, 120)
        .save(p).expect("save failed");
}

fn easy_with_dots(p: &Path) {
    let mut c = Captcha::new();
    c.add_char()
        .add_char()
        .add_char()
        .add_char()
        .add_char()
        .apply_filter(Noise::new(0.3))
        .apply_filter(Grid::new(5, 5))
        .view(220, 120)
        .apply_filter(Dots::new(20))
        .save(p).expect("save failed");
}

fn easy_with_dots_wave(p: &Path) {
    let mut c = Captcha::new();
    c.add_char()
        .add_char()
        .add_char()
        .add_char()
        .add_char()
        .apply_filter(Noise::new(0.4))
        .apply_filter(Wave::new(2.0, 20.0).direction(Direction::HORIZONTAL))
        .apply_filter(Wave::new(2.0, 20.0).direction(Direction::VERTICAL))
        .view(220, 120)
        .apply_filter(Dots::new(15))
        .save(p).expect("save failed");
}

fn main() {
    easy(Path::new("/tmp/captcha1.png"));
    easy_with_dots(Path::new("/tmp/captcha2.png"));
    easy_with_dots_wave(Path::new("/tmp/captcha3.png"));
}
