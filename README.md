# CAPTCHA library written in Rust
[![Build Status](https://travis-ci.org/daniel-e/captcha.svg?branch=master)](https://travis-ci.org/daniel-e/captcha)

A library to generate CAPTCHAs like these:

![captcha](doc/captcha3.png) &nbsp; ![captcha](doc/captcha2.png) &nbsp; ![captcha](doc/captcha_mila_medium.png)

### Requirements

Rust (https://www.rust-lang.org/)

### Documentation

https://docs.rs/captcha

## Usage

Add the following dependency to the `Cargo.toml` file:

```toml
[dependencies]
captcha = "*"
```

In your source file do:

```rust
extern crate captcha;

use captcha::Captcha;
use captcha::filters::Noise;
use std::path::Path;

fn main() {
    Captcha::new()
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(220, 120)
        .save(Path::new("/tmp/captcha.png"))
        .expect("save failed");
}
```

## Running the example

    git clone git@github.com:daniel-e/captcha.git
    cd captcha
    cargo run --example captcha

This example creates the following three CAPTCHA images:
* /tmp/captcha1.png
* /tmp/captcha2.png
* /tmp/captcha3.png

The generated images will look like the following three images:

![captcha](doc/captcha1.png) &nbsp; ![captcha](doc/captcha2.png)
&nbsp;
![captcha](doc/captcha3.png)