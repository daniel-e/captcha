# Building a Release

If not logged in at crates.io yet:

    cargo login

Run some tests:

    cargo build --release
    cargo example --release captcha

Perform the following steps:

* Version in Cargo.toml
* ChangeLog 
* Push all changed
* Create a release on GitHub
* Publish to crates.io via `cargo publish` [details](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
