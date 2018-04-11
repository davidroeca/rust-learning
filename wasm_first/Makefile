PROJECT_NAME="wasm_first"

.PHONY: default
default: all

.PHONY: build-wasm
build-wasm:
	cargo +nightly build --target wasm32-unknown-unknown --release

.PHONY: shrink-wasm
shrink-wasm: build-wasm
	wasm-gc target/wasm32-unknown-unknown/release/$(PROJECT_NAME).wasm \
		-o $(PROJECT_NAME).gc.wasm

.PHONY: all
all: shrink-wasm

.PHONY: serve
serve:
	http
