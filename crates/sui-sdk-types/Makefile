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
	# Note on proptest:
	#
	# Pin to this specific commit in order to work around an issue where proptest doesn't build properly in wasm environments
	# see https://github.com/proptest-rs/proptest/pull/270 for more info
	cargo nextest run --all-features --config 'patch.crates-io.proptest.git="https://github.com/bmwill/proptest.git"' --config 'patch.crates-io.proptest.rev="0a140789d25f4999632b8b88aedb1e2ba056151d"'
	cargo test --doc

.PHONY: wasm
wasm:
	# Note on proptest:
	#
	# Pin to this specific commit in order to work around an issue where proptest doesn't build properly in wasm environments
	# see https://github.com/proptest-rs/proptest/pull/270 for more info
	CC=clang wasm-pack test --node --all-features --config 'patch.crates-io.proptest.git="https://github.com/bmwill/proptest.git"' --config 'patch.crates-io.proptest.rev="0a140789d25f4999632b8b88aedb1e2ba056151d"'

%:
	$(MAKE) -C ../.. $@
