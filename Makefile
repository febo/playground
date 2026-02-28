# Replace all "-" with "/" in the given string.
make-path = $(subst -,/,$1)
# Convert 'programs/anything' to 'anything'.
program-target = $(subst /,-,$(patsubst programs/%,%,$1))
# All files directly inside programs.
PROGRAMS := $(wildcard programs/*/*)
# Generate the dashed target program names.
PROGRAM_TARGETS := $(foreach src,$(PROGRAMS),$(call program-target,$(src)))
# Get the command-line arguments after the target.
ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))

# Available targets.
.PHONY: bench all build clean clippy format

# Run `cargo bench`.
#
# Expected args: <type> [branch name]
bench:
	@cargo bench --bench $(ARGS)

# Build all programs.
all:
	@for dir in $(PROGRAM_TARGETS); do \
		$(MAKE) build-$$dir; \
	done

# Build a program.
build-%:
	@RUSTFLAGS="-C embed-bitcode=yes -C lto=fat" cargo build-sbf --manifest-path programs/$(call make-path,$*)/Cargo.toml --tools-version v1.51

# Run `cargo clean`.
clean:
	@cargo clean

# Run `cargo clippy`.
clippy:
	@cargo clippy \
		--workspace --all-targets -- \
		--deny=warnings \
		--deny=clippy::default_trait_access \
		--deny=clippy::arithmetic_side_effects \
		--deny=clippy::manual_let_else \
		--deny=clippy::used_underscore_binding

# Run `cargo fmt`.
format:
	@cargo fmt --all -- --check

%:
	@# Ignore unknown targets to allow passing arguments after the target.
