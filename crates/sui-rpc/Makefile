# Set the default target of this Makefile
.PHONY: all
all:: clippy test

.PHONY: check-features
check-features:
	cargo hack check --feature-powerset --no-dev-deps

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --doc


.PHONY: update-protos
update-protos:
	rm -rf vendored/sui-apis
	git clone --depth=1 https://github.com/MystenLabs/sui-apis.git vendored/sui-apis
	rm -rf vendored/sui-apis/.git

.PHONY: proto
proto:
	cargo run -p proto-build

%:
	$(MAKE) -C ../.. $@
