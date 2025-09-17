.PHONY: all build clean run test

# No more separate build steps. Cargo handles everything.
all: build
build:
	@echo "Building kernel with integrated bootloader..."
	cargo build --manifest-path kernel/Cargo.toml

clean:
	@echo "Cleaning project..."
	cargo clean --manifest-path kernel/Cargo.toml
	rm -rf target

# `cargo run` now automatically uses QEMU because of our .cargo/config.toml
run:
	cargo run --manifest-path kernel/Cargo.toml

# `cargo test` is not yet configured, so this is a placeholder.
test: build
	@echo "Automated test runner not yet configured for this boot method."
