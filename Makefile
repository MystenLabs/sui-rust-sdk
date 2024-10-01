# Set the default target of this Makefile
.PHONY: all
all:: ci

.PHONY: check-features
check-features:
	$(MAKE) -C crates/sui-sdk-types check-features
	$(MAKE) -C crates/sui-crypto check-features

.PHONY: check-fmt
check-fmt:
	cargo fmt -- --config imports_granularity=Item --check

.PHONY: fmt
fmt:
	cargo fmt -- --config imports_granularity=Item

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --doc

.PHONY: wasm
wasm:
	$(MAKE) -C crates/sui-sdk-types wasm
	$(MAKE) -C crates/sui-crypto wasm

.PHONY: doc
doc:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps

.PHONY: doc-open
doc-open:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps --open

.PHONY: ci
ci: check-features check-fmt test wasm

.PHONY: ci-full
ci-full: ci doc

.PHONY: clean
clean:
	cargo clean

.PHONY: clean-all
clean-all: clean
	git clean -dX
