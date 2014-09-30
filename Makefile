all: build copyshaders

build:
	cargo build

target/media/shaders:
	mkdir -p target/media
	cp -r shaders target/media

copyshaders: target/media/shaders

update:
	cargo update

clean:
	cargo clean

.PHONY: \
	all \
	build \
	clean \
	copyshaders \
	update
