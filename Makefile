check:
	cargo hack check --feature-powerset --no-dev-deps

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-features --all-targets

test:
	cargo test --all-features

wasm:
	CC=clang wasm-pack test --node --all-features

doc:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps

doc-open:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps --open

ci: check fmt-check clippy test wasm

ci-full: ci doc

clean:
	cargo clean

clean-all: clean
	git clean -dX
