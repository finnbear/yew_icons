generate:
	cargo run --release --bin generator --features _generator

submodules:
	git submodule init
	git submodule update

test:
	cargo test --features _testing,lucide