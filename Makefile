generate:
	cargo run --release --bin generator --features generator

submodules:
	git submodule init
	git submodule update