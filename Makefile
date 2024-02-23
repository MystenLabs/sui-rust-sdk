check-features:
	cargo hack check --feature-powerset --no-dev-deps

check-fmt:
	cargo fmt -- --check

clippy:
	cargo clippy --all-features --all-targets

doc:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps

test:
	cargo test --all-features

wasm:
	CC=clang wasm-pack test --node --all-features

ci: check-features check-fmt clippy doc test wasm

clean:
	cargo clean

clean-all: clean
	git clean -dX
