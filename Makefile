all: build

build:
	cargo build

clean:
	cargo clean

.PHONY: \
	all \
	build \
	clean
