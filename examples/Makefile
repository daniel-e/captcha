all:
	cargo run --example captcha
	cargo run --example build_captchas
	cargo run --release --example perf

perf:
	cargo build --release --example perf
	valgrind --tool=callgrind ../target/release/examples/perf
