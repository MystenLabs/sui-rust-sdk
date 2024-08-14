# Set the default target of this Makefile
.PHONY: all
all:: ci-full

.PHONY: check
check:
	cargo hack check --feature-powerset --no-dev-deps

.PHONY: fmt-check
fmt-check:
	cargo fmt -- --check

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo test --all-features

.PHONY: wasm
wasm:
	CC=clang wasm-pack test --node --all-features

.PHONY: doc
doc:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps

.PHONY: doc-open
doc-open:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps --open

.PHONY: ci
ci: check fmt-check clippy test wasm

.PHONY: ci-full
ci-full: ci doc

.PHONY: clean
clean:
	cargo clean

.PHONY: clean-all
clean-all: clean
	git clean -dX
