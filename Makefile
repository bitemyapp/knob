package = knob

env = OPENSSL_INCLUDE_DIR="/usr/local/opt/openssl/include"
cargo = $(env) cargo
debug-env = RUST_BACKTRACE=1 RUST_LOG=$(package)=debug
debug-cargo = $(env) $(debug-env) cargo

build-release:
	$(cargo) build --release

build:
	$(cargo) build

clippy:
	$(cargo) clippy

run: build
	./target/debug/$(package)

install:
	$(cargo) install --force

test:
	$(cargo) test

test-debug:
	$(debug-cargo) test -- --nocapture

fmt:
	$(cargo) fmt

watch:
	$(cargo) watch

.PHONY : build build-release run install test test-debug fmt watch dev-deps
