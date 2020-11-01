extern crate captcha;

use captcha::{gen, Difficulty};
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let captcha = gen(Difficulty::Easy);
    let s = captcha.as_wav();
    let mut c = 0;
    for i in s {
        c += 1;
        let fname = format!("/tmp/audio{}.wav", c);
        let mut f = File::create(fname)?;
        f.write_all(&i.unwrap())?;
    }
    println!("{}", captcha.chars_as_string());
    Ok(())
}
