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
	@# Temporarily move .cargo to avoid using local config during benchmarks.
	@# Always restore it, even if bench fails.
	@if [ -d .cargo-temp ] && [ ! -d .cargo ]; then mv .cargo-temp .cargo; fi
	@if [ -d .cargo ]; then \
		mv .cargo .cargo-temp; \
		trap 'if [ -d .cargo-temp ] && [ ! -d .cargo ]; then mv .cargo-temp .cargo; fi' EXIT; \
		cargo +nightly bench --bench $(ARGS); \
	else \
		cargo +nightly bench --bench $(ARGS); \
	fi

# Build all programs.
all: build

# Build all programs.
build: $(addprefix build-,$(PROGRAM_TARGETS))

# Build a program.
build-%:
	@# Self-heal if a previous bench left `.cargo` renamed.
	@if [ -d .cargo-temp ] && [ ! -d .cargo ]; then mv .cargo-temp .cargo; fi
	@program_path=programs/$(call make-path,$*); \
	config_path=.cargo/config.toml; \
	if [ -f $$program_path/.cargo/config.toml ]; then \
		config_path=$$program_path/.cargo/config.toml; \
	fi; \
	cargo +nightly --config $$config_path build-bpf --manifest-path $$program_path/Cargo.toml

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
