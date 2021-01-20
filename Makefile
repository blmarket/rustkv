all:
	CROSS_COMPILE=/opt/cross/bin/x86_64-linux-musl- cargo build --target x86_64-unknown-linux-musl --release

clean:
	cargo clean

rust.zip:
	zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

dist: rust.zip

.PHONY: dist
