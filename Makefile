build: src/main.rs
	cargo build --release

build-win: src/main.rs
	rustup run nightly cargo build -v --release -Zbuild-std --target x86_64-pc-windows-gnu
