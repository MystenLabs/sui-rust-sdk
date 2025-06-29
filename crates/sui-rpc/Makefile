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
	@rm -rf vendored/sui-apis
	@rm -rf vendored/proto
	@git clone -q --depth=1 https://github.com/MystenLabs/sui-apis.git vendored/sui-apis
	@mv vendored/sui-apis/proto vendored/
	@rm -rf vendored/sui-apis

.PHONY: diff-protos
diff-protos:
	@rm -rf vendored/sui-apis
	@git clone -q --depth=1 https://github.com/MystenLabs/sui-apis.git vendored/sui-apis
	git diff --no-index vendored/sui-apis/proto vendored/proto
	@rm -rf vendored/sui-apis

.PHONY: proto
proto:
	cargo run -p proto-build

%:
	$(MAKE) -C ../.. $@
