all:
	cargo doc --no-deps
	cargo test -- --nocapture
	cargo run --example captcha