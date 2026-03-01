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
	@mv .cargo .cargo-temp 2>/dev/null || true
	@cargo +nightly bench --bench $(ARGS) || true
	@mv .cargo-temp .cargo

# Build all programs.
all: build

# Build all programs.
build: $(addprefix build-,$(PROGRAM_TARGETS))

# Build a program.
build-%:
	@# Not great but avoid to have to manually rename .cargo each time benches fail.
	@mv .cargo-temp .cargo 2>/dev/null || true
	@cargo +nightly build-bpf --manifest-path programs/$(call make-path,$*)/Cargo.toml

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
	$(error Unknown target '$@')
