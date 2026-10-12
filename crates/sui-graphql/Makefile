# Set the default target of this Makefile
.PHONY: all
all:: wasm

.PHONY: wasm
wasm:
	CC=clang cargo check --target wasm32-unknown-unknown

%:
	$(MAKE) -C ../.. $@
