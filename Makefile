# Run `cargo bench`.
#	1) name of the program
#   2) bench name
.PHONY: bench

# Remove the first word (the target) to get positional arguments
ARG1 := $(word 2, $(MAKECMDGOALS))
ARG2 := $(word 3, $(MAKECMDGOALS))

bench:
	cargo bench --bench $(ARG1) -- $(ARG2)

# Build the program.
.PHONY: build
build:
	cargo build-sbf --manifest-path programs/pinocchio/Cargo.toml --tools-version v1.51
	RUSTFLAGS="-C embed-bitcode=yes -C lto=fat" cargo build-sbf --manifest-path programs/sdk/Cargo.toml --tools-version v1.51

# Run `cargo clean`.
.PHONY: clean
clean:
	cargo clean

# Run `cargo clippy`.
.PHONY: clippy
clippy:
	cargo clippy

# Run `cargo fmt`.
.PHONY: format
format:
	cargo fmt

# Catch-all rule to prevent errors when target is not
# match (this happens with command-line arguments)
%:
	@:
