# Building a Release

If not logged in at crates.io yet:

    cargo login

Run some tests:

    cargo build --release
    cargo example --release captcha

Update:

* Version in Cargo.toml
* ChangeLog 

Publish to crates.io

    cargo publish

See https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html for details.
