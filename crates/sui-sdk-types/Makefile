# Set the default target of this Makefile
.PHONY: all
all:: check-features clippy test wasm

.PHONY: check-features
check-features:
	cargo hack check --feature-powerset --no-dev-deps

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --all-features --doc

.PHONY: wasm
wasm:
	CC=clang wasm-pack test --node --all-features

%:
	$(MAKE) -C ../.. $@
