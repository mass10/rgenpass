build:
	cargo fmt
	cargo build --release

install:
	cargo fmt
	cargo build --release
	cargo install --path .

uninstall:
	cargo uninstall rgenpass
